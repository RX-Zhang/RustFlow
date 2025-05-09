
fn maximize_volume_cuboid_given_sum_sides(s: i32) -> i32 {
    let mut maxvalue = 0;
    for i in 1..=s - 2 {
        for j in 1..=s - 1 {
            let k = s - i - j;
            maxvalue = maxvalue.max(i * j * k);
        }
    
    maxvalue
}
