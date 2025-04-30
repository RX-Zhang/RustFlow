

fn square_root_of_an_integer_1(x: i32) -> i32 {
    if x == 0 || x == 1 {
        return x;
    }

    let mut start: i32 = 1;
    let mut end: i32 = x;
    let mut ans: i32 = 0;

    while start <= end {
        let mid = (start + end) / 2;
        if mid * mid == x {
            return mid;
        }
        if mid * mid < x {
            start = mid + 1;
            ans = mid;
        } else {
            end = mid - 1;
        }
    }

    ans
}
