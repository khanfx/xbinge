use rand::Rng;

pub fn create_id() -> String {
    let mut rng = rand::thread_rng();
    let hex_string: String = (0..16)
        .map(|_| format!("{:x}", rng.gen_range(0..16)))
        .collect();
    hex_string
}
