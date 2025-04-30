fn some_function(mut idx: usize, i: usize) -> usize {
    idx = idx.wrapping_add(i);
    idx
}
