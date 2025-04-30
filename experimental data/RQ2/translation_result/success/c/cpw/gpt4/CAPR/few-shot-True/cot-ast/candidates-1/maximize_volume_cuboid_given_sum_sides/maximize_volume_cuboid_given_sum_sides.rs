fn maximize_volume_cuboid_given_sum_sides(s: i32) -> i32 {
    let mut maxvalue: i32 = 0;

    let mut i: i32 = 1;
    while i <= s.wrapping_sub(2) {
        let mut j: i32 = 1;
        while j <= s.wrapping_sub(1) {
            let k = s.wrapping_sub(i).wrapping_sub(j);
            let volume = i.wrapping_mul(j).wrapping_mul(k);
            if volume > maxvalue {
                maxvalue = volume;
            }
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }

    maxvalue
}