//! Find main fuzzing entry

// pub struct MainEntry<'me> {
//     function_symbols: &'me HashMap<String, SignatureData>,
// }

use std::collections::HashMap;  // 键值对数据结构

// 图形处理库
use petgraph::{
    algo::tarjan_scc,
    graph::{DefaultIx, DiGraph, NodeIndex},
};
use syn::visit::Visit; // 解析和操作 Rust 代码的库

use crate::handled_macros;  //lib.rs
use crate::syntax::SignatureData; // syntax.rs

// 从给定的抽象语法树（AST）中，找到程序的主入口函数
pub fn find_main_entry(
    ast: &syn::File, // 抽象语法树的文件结构
    function_symbols: &HashMap<String, SignatureData>,
) -> String {
    // 基于AST和函数符号表生成一个函数调用图
    let call_graph = CallGraph::new(ast, function_symbols);
    // 使用Tarjan算法找到调用图中的强连通分量（SCC），即一组互相调用的函数集合
    let sccs = tarjan_scc(&call_graph.graph);
    // 用于存储顶层函数
    let mut top_levels = vec![];
    // 遍历SCCs
    for scc in sccs.iter().rev() 
    {
        // 过滤有用的节点,将不纯函数的索引加入useful向量。
        // 如果函数是纯函数（无副作用），则跳过。
        let useful = scc
            .iter()
            .filter_map(|node_idx| {
                let symbol = call_graph.map.get(node_idx).unwrap();
                if !function_symbols.get(&symbol[..]).unwrap().pure {
                    // Some(symbol)
                    Some(node_idx)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        // 检查useful向量的长度，即有用函数的数量。
        match useful.len() {
            0 => continue,
            _ => {
                // skip top-level "unused" functions that don't call any other functions
                let node_idx = useful[0]; // 取第一个节点。
                if call_graph.graph.neighbors(*node_idx).count() != 0 {
                    let symbol = call_graph.map.get(node_idx).unwrap();
                    return symbol.to_string();
                } else {
                    top_levels.push(node_idx);
                }
                // return useful[0].to_string()
            } // _ => panic!("mutually recursive functions are not handled at this moment"),
        }
    }
    // 遍历完SCC后，仍然没有找到入口点,就从top_levels中取第一个节点
    if top_levels.len() > 0 {
        let node_idx = top_levels[0];
        let symbol = call_graph.map.get(node_idx).unwrap();
        return symbol.to_string();
    }

    panic!("main entry not found")
}

// 一个图结构，用于表示函数调用关系
struct CallGraph<'name> {
    nodes: HashMap<&'name str, NodeIndex<DefaultIx>>,
    map: HashMap<NodeIndex<DefaultIx>, &'name str>,
    graph: DiGraph<(), ()>,
}

impl<'name> CallGraph<'name> {
    // CallGraph 的构造函数，用于创建 CallGraph 实例。
    fn new(file: &syn::File, function_symbols: &'name HashMap<String, SignatureData>) -> Self {
        let mut graph = DiGraph::new();
        let mut map = HashMap::new();
        let mut nodes = HashMap::new();
        for name in function_symbols.keys() {
            let node = graph.add_node(());
            nodes.insert(&name[..], node);
            map.insert(node, &name[..]);
        }
        let mut call_graph = CallGraph { nodes, map, graph };
        CallGraphBuilder {
            graph: &mut call_graph,
        }
        .visit_file(file);
        call_graph
    }
}

// 用于构建函数调用图。
struct CallGraphBuilder<'me, 'name> {
    graph: &'me mut CallGraph<'name>,
}

impl Visit<'_> for CallGraphBuilder<'_, '_> {
    // 处理每个函数的定义，通过 Inner 结构体遍历函数体中的调用表达式和宏。
    fn visit_item_fn(&mut self, item_fn: &'_ syn::ItemFn) {
        let name = item_fn.sig.ident.to_string();
        let Some(&idx) = self.graph.nodes.get(&name[..]) else {
            return;
        };
        // 用于处理函数体中的调用和宏，更新调用图中的边信息。
        struct Inner<'me, 'name>(NodeIndex<DefaultIx>, &'me mut CallGraph<'name>);
        impl Visit<'_> for Inner<'_, '_> {
            // 处理函数调用表达式（syn::ExprCall,查找调用的函数，并在图中添加边
            fn visit_expr_call(&mut self, expr_call: &'_ syn::ExprCall) {
                match &*expr_call.func {
                    syn::Expr::Path(ref path) => {
                        let path = &path.path;
                        if let Some(symbol) = path.get_ident() {
                            let symbol = symbol.to_string();
                            if let Some(&idx) = self.1.nodes.get(&symbol[..]) {
                                self.1.graph.add_edge(self.0, idx, ());
                            }
                        }
                    }
                    _ => {}
                }
                syn::visit::visit_expr_call(self, expr_call);
            }
            // 处理宏（syn::Macro），检查宏的路径是否需要处理
            fn visit_macro(&mut self, mac: &syn::Macro) {
                let path = &mac.path;
                let path_name = quote::quote!(#path).to_string();
                if handled_macros(&path_name) {
                    let tokens = &mac.tokens;
                    let mock: syn::ExprCall = syn::parse_quote!(mock_macro(#tokens));
                    self.visit_expr_call(&mock);
                }
                syn::visit::visit_macro(self, mac)
            }
        }
        Inner(idx, self.graph).visit_block(&item_fn.block);
    }
}
