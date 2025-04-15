

fn compute_n_under_modulo_p(n: i32, p: i32) -> i32 {
    if n >= p {
        return 0;
    }
    
    let mut result = 1;
    for i in 1..=n {
        result = (result * i) % p;
    }
    
    result
}

fn main() {
    // Call the function here
}
