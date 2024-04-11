use aarya_utils::{
    environ::Environ,
    image_ops::encode_to_base64,
    openai_ops::openai_ops::{prep_header, prep_payload, send_request},
};

#[test]
fn test_image_encoding() {
    match encode_to_base64("../.temp-data/Screenshot 2024-03-17 at 8.22.32 PM.png") {
        Ok(encoded_image) => {
            // print!("{encoded_image}");
            assert!(encoded_image.len() > 0);
        }
        Err(e) => {
            panic!("Error: {}", e);
        }
    }
}

#[tokio::test]
async fn test_openai_api_call() {
    let env = Environ::default();
    let h = prep_header(env.openai_key).unwrap();

    let encoded_image = match encode_to_base64("../.temp-data/Screenshot 2024-03-17 at 8.22.32 PM.png") {
        Ok(encoded_image) => encoded_image,
        Err(e) => {
            panic!("Error: {}", e);
        }
    };

    let payload = prep_payload(encoded_image, "Explain the contents of this image".to_string());

    match send_request(h, payload).await {
        Ok(response) => {
            println!("Response received: {}", response);
            assert!(!response.contains("error"));
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            assert!(false, "Request failed");
        }
    }
}
