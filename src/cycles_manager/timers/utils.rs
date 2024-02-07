pub fn nearest_past_day_start(time_nano: u64) -> u64 {
    const NANO_PER_DAY: u64 = 86_400_000_000_000;
    let remainder = time_nano % NANO_PER_DAY;
    let nearest_day_start = time_nano - remainder;
    return nearest_day_start;
}

pub fn next_midnight_time() -> u64 {
    let time_now: u64 = ic_cdk::api::time();
    let last_midnight: u64 = nearest_past_day_start(time_now);
    let next_midnight: u64 = last_midnight + 86_400_000_000_000;
    return next_midnight;
}
