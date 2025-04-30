
fn maximize_volume_cuboid_given_sum_sides(s: i32) -> i32 {
    let mut maxvalue = 0;
    for i in 1..=s.wrapping_sub(2) {
        for j in 1..=s.wrapping_sub(1) {
            let k = s.wrapping_sub(i).wrapping_sub(j);
            maxvalue = maxvalue.max(i * j * k);
        }
    }
    maxvalue
}
