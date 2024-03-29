use crc32fast::Hasher;

pub fn hash(input: &str) -> String {
	let mut hasher = Hasher::new();
	hasher.update(input.as_bytes());
	let checksum = hasher.finalize();
	format!("{:08x}", checksum)
}