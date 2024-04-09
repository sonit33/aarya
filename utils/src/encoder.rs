use url::form_urlencoded::{ byte_serialize, parse };

pub struct UrlEncoderDecoder;

impl UrlEncoderDecoder {
    // Encodes a string into application/x-www-form-urlencoded format
    pub fn encode(input: &str) -> String {
        byte_serialize(input.as_bytes()).collect()
    }

    // Decodes a string from application/x-www-form-urlencoded format
    pub fn decode(input: &str) -> Result<String, std::str::Utf8Error> {
        Ok(
            parse(input.as_bytes())
                .map(|(key, _)| key)
                .collect::<Vec<_>>()
                .join("")
        )
    }
}
