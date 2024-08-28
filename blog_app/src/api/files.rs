use std::{collections::HashMap, fs::File, io::Write};

use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{post, HttpResponse, Responder};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

use crate::utils::random_ops;

#[derive(Debug, Deserialize, Serialize)]
pub struct FileResponse {
    pub paths: HashMap<String, String>,
}

#[post("/api/markdown")]
pub async fn post_markdown(mut payload: Multipart) -> impl Responder {
    let mut response = Vec::new();

    if let Some(item) = payload.next().await {
        let unique_key = random_ops::generate_guid(16);

        // Create the file path with the unique key and original extension
        let file_path = format!("./assets/markdowns/{}.md", unique_key);

        response.push(process_file(item, &file_path, &unique_key).await);
    }

    HttpResponse::Ok().json(response)
}

async fn process_file(
    item: Result<Field, MultipartError>,
    file_path: &String,
    unique_key: &String,
) -> FileResponse {
    let mut field = item.unwrap();
    // Create a new file and write the contents
    let mut f = File::create(file_path).unwrap();
    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();
        f.write_all(&data).unwrap();
    }

    // Append the unique key and extension to the response
    FileResponse {
        paths: HashMap::from([("markdown".to_string(), unique_key.to_string())]),
    }
}
