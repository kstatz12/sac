use rand::distributions::Uniform;
use rand::Rng;

pub fn configure_delay(base: u32) -> u64 {
    let variance = get_variance(20, 30);
    let d = match get_direction(variance) {
        true => base + variance,
        false => base - variance,
    };
    return (d * 60) as u64;
}

pub fn get_direction(n: u32) -> bool {
    return n % 2 == 0;
}

pub fn get_variance(l: u32, u: u32) -> u32 {
    let mut rng = rand::thread_rng();
    let range = Uniform::new_inclusive(l, u);
    return rng.sample(range);
}
