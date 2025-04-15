
#[allow(unused_variables)]
#[allow(dead_code)]

fn maximize_volume_cuboid_given_sum_sides(s: i32) -> i32 {
    let mut maxvalue: i32 = 0;
    for i in 1..(s - 2 + 1) {
        for j in 1..(s - 1 + 1) {
            let k: i32 = s - i - j;
            maxvalue = std::cmp::max(maxvalue, i * j * k);
        }
    }
    return maxvalue;
}
