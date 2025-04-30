//! A Parser for C data

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    // see the "streaming/complete" paragraph lower for an explanation of these submodules
    character::complete::{alpha1, alphanumeric1, char, crlf, newline, tab},
    combinator::{all_consuming, map, opt, recognize, value},
    error::ParseError,
    multi::{many0, many0_count, many1, many1_count, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

use crate::{CFnSig, CStruct};

// 用于解析合法的标识符（identifier），即以字母或下划线开头，后面可以跟随字母、数字或下划线。
fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(input)
}

// 用于解析数组声明符，即方括号包围的表达式（如 [10] 或 []）。
fn array_declarator(input: &str) -> IResult<&str, Vec<String>> {
    many0(delimited(
        token(char('[')),
        map(recognize(opt(is_not("]"))), |expr: &str| expr.to_owned()),
        token(char(']')),
    ))(input)
}

// 用于解析设计符，如 const、static、inline 这类修饰符。
fn designator(input: &str) -> IResult<&str, &str> {
    alt((tag("const"), tag("static"), tag("inline")))(input)
}

// 用于解析类型修饰符，如 signed、unsigned、long、short 等。
fn modifier(input: &str) -> IResult<&str, &str> {
    alt((tag("signed"), tag("unsigned"), tag("long"), tag("short")))(input)
}

// 解析各种基本的算术类型，如 int8_t、int16_t、int、float、double 等。
fn arithmetic_type(input: &str) -> IResult<&str, &str> {
    alt((
        tag("int8_t"),
        tag("int16_t"),
        tag("int32_t"),
        tag("int64_t"),
        tag("char"),
        tag("int"),
        tag("float"),
        tag("double"),
    ))(input)
}

// 解析带修饰符的基本类型，例如 unsigned int、long double 等。
fn primitive_type(input: &str) -> IResult<&str, String> {
    alt((
        map(
            alt((
                pair(many1(token(modifier)), opt(token(arithmetic_type))),
                pair(many0(token(modifier)), map(token(arithmetic_type), Some)),
            )),
            |(modifiers, opt_type)| {
                modifiers
                    .iter()
                    .copied()
                    .chain(opt_type)
                    .map(|s| s.to_owned())
                    .collect::<Vec<_>>()
                    .join(" ")
            },
        ),
        map(token(tag("bool")), |s: &str| s.to_owned()),
    ))(input)
}

// 解析类型别名或复杂类型（如结构体或枚举类型）。
fn type_alias_or_complex(input: &str) -> IResult<&str, String> {
    alt((
        map(
            preceded(terminated(tag("struct"), dont_care1), identifier),
            |s| format!("struct {s}"),
        ),
        map(
            preceded(terminated(tag("enum"), dont_care1), identifier),
            |s| format!("enum {s}"),
        ),
        map(identifier, |s| s.to_owned()),
    ))(input)
}

// 解析一个完整的类型声明，包括修饰符、基础类型、const 关键字，以及指针符号 *。
fn type_name(input: &str) -> IResult<&str, String> {
    map(
        tuple((
            opt(token(designator)),
            alt((token(primitive_type), token(type_alias_or_complex))),
            opt(token(tag("const"))),
            many0(token(char('*'))),
        )),
        |(_, mut ty, _, opt_star)| {
            // opt_star.map(|_| ty.push('*'));
            opt_star.into_iter().for_each(|_| ty.push('*'));
            ty
        },
    )(input)
}

// 解析字段定义，生成 (String, Vec<(String, Vec<String>)>) 类型的输出。
/// Procuce field_name: type_name
fn field_def(input: &str) -> IResult<&str, (String, Vec<(String, Vec<String>)>)> {
    terminated(
        tuple((
            token(type_name),
            separated_list1(token(char(',')), pair(map(token(identifier), |ident| ident.to_owned()), array_declarator))
        )),
        token(char(';')),
    )(input)
}

// 解析行尾注释（// 开头的注释，直到行结束）。
fn peol_comment<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
    value(
        (), // Output is thrown away.
        pair(tag("//"), is_not("\n\r")),
    )(input)
}

// 解析内联注释（/* 和 */ 包围的注释内容）。
fn pinline_comment<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
    value(
        (), // Output is thrown away.
        tuple((tag("/*"), take_until("*/"), tag("*/"))),
    )(input)
}

// 跳过代码中的不关心的部分（如空白字符、换行符和注释），并不关心这些内容的实际值。
// 1 跳过零个或多个不关心的部分，包括空格、换行符、制表符、行尾注释和内联注释。
fn dont_care0<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
    value(
        (),
        many0_count(alt((
            value((), alt((char(' '), newline, tab))),
            value((), crlf),
            peol_comment,
            pinline_comment,
        ))),
    )(input)
}

// 2 要求至少要跳过一个不关心的部分。
fn dont_care1<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
    value(
        (),
        many1_count(alt((
            value((), alt((char(' '), newline, tab))),
            value((), crlf),
            peol_comment,
            pinline_comment,
        ))),
    )(input)
}

// 创建一个解析器，它在实际解析目标内容之前和之后自动跳过不相关的部分（例如空格、换行符、注释等）
fn token<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(dont_care0, inner, dont_care0)
}

// 用于解析 C 语言中的 typedef struct 语法，并将其转换为一个自定义的 CStruct 结构体。
fn struct_typedef_impl(input: &str) -> IResult<&str, CStruct> {
    map(
        delimited(
            tuple((token(tag("typedef")), token(tag("struct")))),
            tuple((
                opt(token(identifier)),
                delimited(token(char('{')), many0(field_def), token(char('}'))),
                token(identifier),
            )),
            token(char(';')),
        ),
        |(_, fields, ident)| {
            let fields = fields.into_iter().flat_map(move |(ty, fields)| {
                fields.into_iter().map(move |(name, array_decls)| {
                    (name, ty.clone(), array_decls)
                })
            }).collect::<Vec<_>>();
            CStruct {
                ident: ident.to_owned(),
                fields
            }
        },
    )(input)
}

// 用来解析 C 语言风格的结构体声明，并将其转换为一个 CStruct 结构体。
fn struct_decl_impl(input: &str) -> IResult<&str, CStruct> {
    println!("struct_decl: attempting {input}");
    map(
        delimited(
            token(tag("struct")),
            pair(
                token(identifier),
                delimited(token(char('{')), many0(field_def), token(char('}'))),
            ),
            token(char(';')),
        ),
        |(ident, fields)| {
            let fields = fields.into_iter().flat_map(move |(ty, fields)| {
                fields.into_iter().map(move |(name, array_decls)| {
                    (name, ty.clone(), array_decls)
                })
            }).collect::<Vec<_>>();
            CStruct {
                ident: ident.to_owned(),
                fields
            }
        },
    )(input)
}

// 用于解析输入字符串中的 C 结构体定义，并生成一个 CStruct 对象。
pub fn struct_def(input: &str) -> IResult<&str, CStruct> {
    (all_consuming(alt((struct_typedef_impl, struct_decl_impl))))(input)
}

// 用于解析 C 函数的声明，并生成一个 CFnSig 对象。它处理函数的返回类型、函数名、参数列表和函数声明的结束符号。
fn fn_decl_impl(input: &str) -> IResult<&str, CFnSig> {
    map(
        terminated(
            tuple((
                token(type_name),
                token(identifier),
                delimited(
                    token(char('(')),
                    separated_list0(
                        token(char(',')),
                        pair(
                            token(type_name),
                            preceded(opt(token(identifier)), array_declarator),
                        ),
                    ),
                    token(char(')')),
                ),
            )),
            token(char(';')),
        ),
        |(ret, ident, args)| CFnSig {
            ident: ident.to_owned(),
            args,
            ret,
        },
    )(input)
}

// 用于解析输入字符串中的 C 函数签名，并生成一个 CFnSig 对象。
pub fn fn_sig(input: &str) -> IResult<&str, CFnSig> {
    (all_consuming(fn_decl_impl))(input)
}

// 确保你的解析器能够正确处理各种 C 语言定义的场景，
// 包括类型定义、结构体声明、函数签名等。通过这些测试，可以验证解析器的鲁棒性和正确性。
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert!(type_name("static unsigned long long int")
            .is_ok_and(|(remaining, _)| remaining.is_empty()));

        assert!(type_name("float").is_ok_and(|(remaining, _)| remaining.is_empty()));

        assert!(field_def("  int  *  qeofuef_teftbGEGTE_    ;")
            .is_ok_and(|(remaining, _)| remaining.is_empty()));

        assert!(field_def("  wchar_t    qeofuef_teftbGEGTE_    ;")
            .is_ok_and(|(remaining, _)| remaining.is_empty()));
        assert!(
            struct_def("typedef struct url_parse { int f; } url_parse_t ;")
                .is_ok_and(|(remaining, _)| remaining.is_empty())
        );
        assert!(
            struct_def("typedef struct { int f;    char*   g  ;  int    *h; } url_parse_t ;")
                .is_ok_and(|(remaining, _)| { remaining.is_empty() })
        );
        assert!(
            struct_def("struct url_parse_t { int f;    char*   g  ;  int    *h; }  ;")
                .is_ok_and(|(remaining, _)| { remaining.is_empty() })
        );
        assert!(fn_sig("char  * foobar  (int x,   char  *   , bool );")
            .is_ok_and(|(remaining, _)| remaining.is_empty()));

        assert!(fn_sig("char*   foobar  (int x,   char  *   , bool );")
            .is_ok_and(|(remaining, _)| remaining.is_empty()));

        assert!(fn_sig(
            "void add_child16(art_node16 *n, art_node **ref, unsigned char c, void *child) ;"
        )
        .is_ok());

        assert!(struct_def(
            "struct json_parse_state_s {
                const char *src;
                size_t size;
                size_t offset;
                size_t flags_bitset;
                char *data;
                char *dom;
                size_t dom_size;
                size_t data_size;
                size_t line_no;     /* line counter for error reporting. */
                size_t line_offset; /* (offset-line_offset) is the character number (in
                                       bytes). */
                size_t error;
              };"
        )
        .is_ok());

        assert!(field_def("unsigned char modulator_40, carrier_40;").is_ok());

        assert!(struct_typedef_impl(
            "typedef struct opl_timbre_t {
                unsigned long modulator_E862, carrier_E862;
                unsigned char modulator_40, carrier_40;
                unsigned char feedconn;
                signed char finetune;
                unsigned char notenum;
                signed short noteoffset;
              } opl_timbre_t;"
        ).is_ok());

        assert!(fn_sig("int json_skip_whitespace(struct json_parse_state_s *state) ;").is_ok());
    }
}
