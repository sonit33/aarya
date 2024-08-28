use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

use std::fmt;

pub const UPLOADS_DIR: &str = "./assets/images/uploads";
pub const RELATIVE_PATH: &str = "/assets/images/uploads";

#[derive(Clone, Serialize, Deserialize)]
pub enum ImageSize {
    Original,
    Thumbnail,
    Profile,
    Wide,
    Hero,
}

impl ImageSize {
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            ImageSize::Thumbnail => (256, 256),
            ImageSize::Profile => (640, 640 * 9 / 16),
            ImageSize::Wide => (800, 800 * 9 / 16),
            ImageSize::Hero => (1260, 1260 * 9 / 16),
            ImageSize::Original => (0, 0),
        }
    }

    pub fn get_display_sizes() -> Vec<ImageSize> {
        vec![
            ImageSize::Thumbnail,
            ImageSize::Profile,
            ImageSize::Wide,
            ImageSize::Hero,
        ]
    }

    pub fn get_all_sizes() -> Vec<ImageSize> {
        vec![
            ImageSize::Original,
            ImageSize::Thumbnail,
            ImageSize::Profile,
            ImageSize::Wide,
            ImageSize::Hero,
        ]
    }
}

impl fmt::Display for ImageSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImageSize::Thumbnail => write!(f, "Thumbnail"),
            ImageSize::Profile => write!(f, "Profile"),
            ImageSize::Wide => write!(f, "Wide"),
            ImageSize::Hero => write!(f, "Hero"),
            ImageSize::Original => write!(f, "Original"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImagePath {
    pub key: String,
    pub extension: String,
}

impl Display for ImagePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.original_path())
    }
}

impl ImagePath {
    pub fn new(key: String, extension: String) -> Self {
        ImagePath { key, extension }
    }

    pub fn get_all_paths(&self) -> HashMap<String, String> {
        ImageSize::get_all_sizes()
            .iter()
            .map(|size| (size.to_string(), self.from_r(size)))
            .collect::<HashMap<String, String>>()
    }

    pub fn get_display_paths(&self) -> HashMap<String, String> {
        ImageSize::get_display_sizes()
            .iter()
            .map(|size| (size.to_string(), self.from_r(size)))
            .collect::<HashMap<String, String>>()
    }

    pub fn original_path(&self) -> String {
        format!("{}/{}.{}", UPLOADS_DIR, self.key, self.extension)
    }

    pub fn thumbnail_path(&self) -> String {
        format!("{}/{}-thumb.{}", UPLOADS_DIR, self.key, self.extension)
    }

    pub fn profile_path(&self) -> String {
        format!("{}/{}-profile.{}", UPLOADS_DIR, self.key, self.extension)
    }

    pub fn wide_path(&self) -> String {
        format!("{}/{}-wide.{}", UPLOADS_DIR, self.key, self.extension)
    }

    pub fn hero_path(&self) -> String {
        format!("{}/{}-hero.{}", UPLOADS_DIR, self.key, self.extension)
    }

    pub fn original_r_path(&self) -> String {
        format!("{}/{}.{}", RELATIVE_PATH, self.key, self.extension)
    }

    pub fn thumbnail_r_path(&self) -> String {
        format!("{}/{}-thumb.{}", RELATIVE_PATH, self.key, self.extension)
    }

    pub fn profile_r_path(&self) -> String {
        format!("{}/{}-profile.{}", RELATIVE_PATH, self.key, self.extension)
    }

    pub fn wide_r_path(&self) -> String {
        format!("{}/{}-wide.{}", RELATIVE_PATH, self.key, self.extension)
    }

    pub fn hero_r_path(&self) -> String {
        format!("{}/{}-hero.{}", RELATIVE_PATH, self.key, self.extension)
    }

    pub fn from(&self, size: &ImageSize) -> String {
        match size {
            ImageSize::Thumbnail => self.thumbnail_path(),
            ImageSize::Profile => self.profile_path(),
            ImageSize::Wide => self.wide_path(),
            ImageSize::Hero => self.hero_path(),
            ImageSize::Original => self.original_path(),
        }
    }

    pub fn from_r(&self, size: &ImageSize) -> String {
        match size {
            ImageSize::Thumbnail => self.thumbnail_r_path(),
            ImageSize::Profile => self.profile_r_path(),
            ImageSize::Wide => self.wide_r_path(),
            ImageSize::Hero => self.hero_r_path(),
            ImageSize::Original => self.original_r_path(),
        }
    }

    pub fn from_string(path: &str) -> Self {
        let parts: Vec<&str> = path.split('/').collect();
        let file_name: Vec<&str> = parts[parts.len() - 1].split('.').collect();
        let key_parts: Vec<&str> = file_name[0].split('-').collect();

        ImagePath {
            key: key_parts[0].to_string(),
            extension: file_name[1].to_string(),
        }
    }
}
