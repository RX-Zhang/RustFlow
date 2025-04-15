
use std::cmp;

fn calculate_angle_hour_hand_minute_hand(h: f64, m: f64) -> i32 {
    if h < 0.0 || m < 0.0 || h > 12.0 || m > 60.0 {
        println!("Wrong input");
    }

    let mut h = h;
    let mut m = m;

    if h == 12.0 {
        h = 0.0;
    }
    if m == 60.0 {
        m = 0.0;
    }

    let hour_angle = 0.5 * (h * 60.0 + m);
    let minute_angle = 6.0 * m;
    let angle = (hour_angle - minute_angle).abs() as i32;
    cmp::min(360 - angle, angle)
}
