
use std::cmp;
use std::num::Wrapping;

fn calculate_angle_hour_hand_minute_hand(h: f64, m: f64) -> i32 {
    if h < 0.0 || m < 0.0 || h > 12.0 || m > 60.0 {
        println!("Wrong input");
    }

    let h = if h == 12.0 { 0.0 } else { h };
    let m = if m == 60.0 { 0.0 } else { m };

    let hour_angle = Wrapping((0.5 * (h * 60.0 + m)) as i32);
    let minute_angle = Wrapping((6.0 * m) as i32);

    let angle = (hour_angle - minute_angle).0.abs();
    cmp::min(360 - angle, angle)
}
