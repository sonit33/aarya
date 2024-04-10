use aarya_utils::image_ops::encode_to_base64;

#[test]
fn test_image_encoding() {
    match encode_to_base64("../.temp-data/Screenshot 2024-03-17 at 8.22.32 PM.png") {
        Ok(encoded_image) => {
            print!("{encoded_image}");
            assert!(encoded_image.len() > 0);
        }
        Err(e) => {
            panic!("Error: {}", e);
        }
    }
}
