fn calculate_angle_hour_hand_minute_hand(h: f64, m: f64) -> i32 {
    if h < 0.0 || m < 0.0 || h > 12.0 || m > 60.0 {
        println!("Wrong input");
    }
    let mut h_adj = h;
    let mut m_adj = m;
    if h_adj == 12.0 {
        h_adj = 0.0;
    }
    if m_adj == 60.0 {
        m_adj = 0.0;
    }
    let hour_angle: i32 = (0.5 * (h_adj * 60.0 + m_adj)) as i32;
    let minute_angle: i32 = 6_i32.wrapping_mul(m_adj as i32);
    let mut angle: i32 = (hour_angle - minute_angle).abs();
    if angle > 180 {
        angle = 360 - angle;
    }
    angle
}