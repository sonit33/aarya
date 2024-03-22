use rand::{distributions::Alphanumeric, Rng};

pub fn generate_guid(chars: usize) -> String {
    let rand_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(chars)
        .map(char::from)
        .collect();
    format!("{}", rand_string)
}
