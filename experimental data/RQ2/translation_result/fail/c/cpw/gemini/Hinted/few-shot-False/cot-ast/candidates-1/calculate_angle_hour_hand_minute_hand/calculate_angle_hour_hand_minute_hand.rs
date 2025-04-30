
fn calculate_angle_hour_hand_minute_hand(h: f64, m: f64) -> i32 {
    if h < 0.0 || m < 0.0 || h > 12.0 || m > 60.0 {
        panic!("Wrong input");
    }
    let mut hour_angle = 0.5 * (h * 60.0 + m);
    if h == 12.0 {
        hour_angle = 0.0;
    }
    let mut minute_angle = 6.0 * m;
    if m == 60.0 {
        minute_angle = 0.0;
    }
    let angle = (hour_angle - minute_angle).abs();
    angle.min(360.0 - angle) as i32
}
