fn write_you_own_power_without_using_multiplication_and_division(a: i32, b: i32) -> i32 {
    if b == 0 {
        return 1;
    }
    let mut answer: i32 = a;
    let mut increment: i32 = a;
    for i in 1..b {
        for _j in 1..a {
            answer = answer.wrapping_add(increment);
        }
        increment = answer;
    }
    answer
}