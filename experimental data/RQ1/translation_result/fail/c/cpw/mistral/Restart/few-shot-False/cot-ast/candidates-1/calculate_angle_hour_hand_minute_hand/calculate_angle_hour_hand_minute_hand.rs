
use std::cmp::min;
use std::i32;

fn calculate_angle_hour_hand_minute_hand(h: f64, m: f64) -> i32 {
    if h < 0.0 || m < 0.0 || h > 12.0 || m > 60.0 {
        println!("Wrong input");
        return -1;
    }

    let hour_angle = (h * 60.0 + m) * 0.5;
    let minute_angle = m * 6.0;

    let angle = (hour_angle - minute_angle).abs() as i32;

    min(360 - angle, angle)
}
