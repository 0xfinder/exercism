// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let mut rate = 1.0;

    if speed >= 5 && speed <= 8 {
        rate = 0.9
    } else if speed >= 9 && speed <= 10 {
        rate = 0.77
    }

    speed as f64 * 221.0 * rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
