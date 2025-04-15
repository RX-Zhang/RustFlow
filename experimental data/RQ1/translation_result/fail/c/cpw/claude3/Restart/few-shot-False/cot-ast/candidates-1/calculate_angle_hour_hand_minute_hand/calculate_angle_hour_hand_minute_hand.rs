
fn calculate_angle_hour_hand_minute_hand(h: f64, m: f64) -> i32 {
    if h < 0.0 || m < 0.0 || h > 12.0 || m > 60.0 {
        println!("Wrong input");
    }

    let mut hour = h;
    let mut minute = m;

    if (hour - 12.0).abs() < f64::EPSILON {
        hour = 0.0;
    }
    if (minute - 60.0).abs() < f64::EPSILON {
        minute = 0.0;
    }

    let hour_angle = (0.5 * (hour * 60.0 + minute)) as i32;
    let minute_angle = (6.0 * minute) as i32;
    let angle = (hour_angle - minute_angle).abs();
    
    std::cmp::min(360 - angle, angle)
}
