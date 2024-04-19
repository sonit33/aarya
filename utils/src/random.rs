use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn generate_guid(chars: usize) -> String {
    let rand_string: String = rand::thread_rng().sample_iter(&Alphanumeric).take(chars).map(char::from).collect();
    rand_string.to_uppercase().to_string()
}

pub fn generate_numbers(from: u8, to: u8) -> u8 {
    let mut rng = thread_rng();

    rng.gen_range(from..=to)
}
