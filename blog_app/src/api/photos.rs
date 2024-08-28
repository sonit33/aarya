use std::{collections::HashMap, ffi::OsStr, fs::File, io::Write, path::Path};

use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{post, HttpResponse, Responder};
use futures::StreamExt;
use image::{imageops::FilterType, GenericImageView};
use serde::{Deserialize, Serialize};

use crate::utils::{
    image_ops::{ImagePath, ImageSize},
    random_ops,
};

#[derive(Deserialize, Serialize)]
pub struct PhotoResponse {
    pub paths: Option<HashMap<String, String>>,
}

#[post("/api/photo")]
pub async fn post_photo(mut payload: Multipart) -> impl Responder {
    let mut response = Vec::new();

    if let Some(item) = payload.next().await {
        response.push(process_photo(item).await);
    }

    HttpResponse::Ok().json(response)
}

#[post("/api/photos")]
pub async fn post_photos(mut payload: Multipart) -> impl Responder {
    let mut response = Vec::new();
    while let Some(item) = payload.next().await {
        response.push(process_photo(item).await);
    }

    // Return the response as JSON
    HttpResponse::Ok().json(response)
}

async fn process_photo(item: Result<Field, MultipartError>) -> PhotoResponse {
    let mut field = item.unwrap();
    let content_disposition = field.content_disposition().clone();
    let filename = content_disposition.get_filename().unwrap();
    let extension = Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("");

    // Generate a unique key
    let unique_key = random_ops::generate_guid(16);

    // Create the file path with the unique key and original extension
    let image_path = ImagePath::new(unique_key.to_string(), extension.to_string());

    // Create a new file and write the contents
    let mut f = File::create(image_path.original_path()).unwrap();
    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();
        f.write_all(&data).unwrap();
    }

    generate_photo_sizes(&image_path);

    // Append the unique key and extension to the response
    PhotoResponse {
        paths: Some(image_path.get_all_paths()),
    }
}

fn generate_photo_sizes(image_path: &ImagePath) {
    // Iterate over all image sizes and create resized photos
    for size in ImageSize::get_display_sizes() {
        resize_photo(image_path, &size);
    }
}

fn resize_photo(image_path: &ImagePath, size: &ImageSize) {
    let path = image_path;
    let original_path = path.original_path();
    let resized_path = path.from(size);
    let (width, height) = size.dimensions();

    // do not proceed if the resized image already exists
    if Path::new(&resized_path).exists() {
        return;
    }

    // Open the original image
    let img = image::open(original_path).unwrap();

    // Calculate the new dimensions while maintaining the aspect ratio
    let (orig_width, orig_height) = img.dimensions();
    let aspect_ratio = orig_width as f32 / orig_height as f32;
    let (new_width, new_height) = if width as f32 / height as f32 > aspect_ratio {
        ((height as f32 * aspect_ratio) as u32, height)
    } else {
        (width, (width as f32 / aspect_ratio) as u32)
    };

    // Resize the image
    let resized_img = img.resize(new_width, new_height, FilterType::Lanczos3);

    // Save the resized image to disk
    let mut output = File::create(&resized_path).unwrap();
    resized_img
        .write_to(
            &mut output,
            image::ImageFormat::from_extension(image_path.extension.clone()).unwrap(),
        )
        .unwrap();
}
