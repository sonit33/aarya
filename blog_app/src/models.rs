use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostThumbnailModel {
    pub title: String,
    pub subtitle: String,
    pub image_url: String,
    pub author: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthorThumbnailModel {
    pub name: String,
    pub image_url: String,
    pub profile_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagThumbnailModel {
    pub name: String,
    pub posts: Vec<PostThumbnailModel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeywordThumbnailModel {
    pub name: String,
    pub posts: Vec<PostThumbnailModel>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TagModel {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostResponseModel {
    pub title: String,
    pub subtitle: String,
    pub body: String,
    pub description: String,
    pub keywords: String,
    pub tldr: String,
    pub hero_image: String,
    pub published: String,
    pub author: AuthorThumbnailModel,
    pub tags: Vec<TagModel>,
    pub tag_thumbnails: Vec<TagThumbnailModel>,
    pub keyword_thumbnails: Vec<KeywordThumbnailModel>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndexPostImageResponseModel {
    pub author: AuthorThumbnailModel,
    pub is_featured: bool,
    pub featured_image: String,
    pub thumbnail_image: String,
    pub tag: TagModel,
    pub title: String,
    pub subtitle: String,
    pub date_published: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndexPostTextResponseModel {
    pub author: AuthorThumbnailModel,
    pub tag: TagModel,
    pub title: String,
    pub subtitle: String,
    pub date_published: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostByTagsResponseModel {
    pub tag: TagModel,
    // featured post (first)
    pub featured_post: IndexPostImageResponseModel,
    // three posts in the side panel (next three posts)
    pub posts: Vec<IndexPostImageResponseModel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexResponseModel {
    pub title: String,
    pub hero_post: IndexPostImageResponseModel,
    pub hero_posts: Vec<IndexPostImageResponseModel>,
    pub featured_posts: Vec<IndexPostImageResponseModel>,
    pub latest_posts: Vec<IndexPostTextResponseModel>,
    pub posts_by_tags: Vec<PostByTagsResponseModel>,
    pub trending_posts: Vec<IndexPostTextResponseModel>,
}
