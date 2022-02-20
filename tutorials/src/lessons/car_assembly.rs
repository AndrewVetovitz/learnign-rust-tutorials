/*
1 to 4: 100% success rate.
5 to 8: 90% success rate.
9 and 10: 77% success rate.
*/

const PRODUCTION_SPEED: f64 = 221.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    return PRODUCTION_SPEED * (speed as f64) * match speed {
        0 => 0.0,
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=u8::MAX => 0.77
    };
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    return (production_rate_per_hour(speed) / 60.0) as u32;
}
