use rand::{distributions::Alphanumeric, Rng};

pub fn generate_guid() -> String {
    let rand_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    format!("{}", rand_string)
}
