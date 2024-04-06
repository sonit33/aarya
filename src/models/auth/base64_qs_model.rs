use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct Base64QuerystringModel {
    pub q: String,
}

pub fn extract_values(s: &str) -> (Option<&str>, Option<&str>) {
    let mut e = None;
    let mut t = None;

    for param in s.split('&') {
        let mut parts = param.split('=');
        match (parts.next(), parts.next()) {
            (Some("e"), Some(value)) => {
                e = Some(value);
            }
            (Some("t"), Some(value)) => {
                t = Some(value);
            }
            _ => {}
        }
    }

    (e, t)
}
