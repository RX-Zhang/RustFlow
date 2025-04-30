
fn zeckendorfs_theorem_non_neighbouring_fibonacci_representation(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return n;
    }
    let mut f1: i32 = 0;
    let mut f2: i32 = 1;
    let mut f3: i32 = 1;
    while f3 <= n {
        f1 = f2;
        f2 = f3;
        f3 = f1.wrapping_add(f2);
    }
    f2
}
