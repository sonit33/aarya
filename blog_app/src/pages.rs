use std::collections::HashMap;

use aarya_utils::hash_ops::string_hasher;
use actix_web::{get, web, HttpResponse, Responder};
use handlebars::Handlebars;
use models::{
    blogs::{PostEntity, PostQueryModel},
    result_types::EntityResult,
};
use pulldown_cmark::{html, Parser};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::MySqlPool;

#[get("/")]
pub async fn home_page(
    handlebars: web::Data<Handlebars<'_>>,
    pool: web::Data<MySqlPool>,
) -> impl Responder {
    // load blog posts
    let post = PostEntity::default();
    match post.find(&pool).await {
        EntityResult::Success(p) => {
            render_template!(handlebars, "index", json!({"title": "Aarya blog", "posts": p}))
        }
        EntityResult::Error(e) => HttpResponse::InternalServerError().body(format!("Error retrieving posts: [{:?}]", e)),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostThumbnailModel {
    pub title: String,
    pub subtitle: String,
    pub image_url: String,
    pub author: String,
}

#[derive(Debug, Serialize, Deserialize)]
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
    pub keyword: String,
    pub posts: Vec<PostThumbnailModel>,
}

#[derive(Debug, Serialize, Deserialize)]
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
    pub keywords: HashMap<String, String>,
    pub tldr: String,
    pub hero_image: String,
    pub published: String,
    pub author: AuthorThumbnailModel,
    pub tags: Vec<TagModel>,
    pub tag_thumbnails: Vec<TagThumbnailModel>,
    pub keyword_thumbnails: Vec<KeywordThumbnailModel>,
}

#[get("/{date_published}/{title}")]
pub async fn post_page(
    handlebars: web::Data<Handlebars<'_>>,
    pool: web::Data<MySqlPool>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (date_published, title) = path.into_inner();
    let post_hash = string_hasher(format!("{}/{}", date_published, title).as_str());
    let post = PostEntity {
        post_hash: Some(post_hash),
        ..Default::default()
    };

    match post.find_post(&pool).await {
        EntityResult::Success(p) => {
            let mut html = String::new();
            html::push_html(&mut html, Parser::new(&p.post_body));
            let post = PostQueryModel { post_body: html, ..p };

            let response_model = PostResponseModel {
                title: post.post_title,
                subtitle: post.post_subtitle,
                body: post.post_body,
                description: post.post_description,
                keywords: post.post_keywords.split(',').map(|x| (x.to_string(), x.to_string())).collect(),
                tldr: post.post_tldr,
                hero_image: post.post_hero_image_url,
                published: post.post_timestamp.format("%B %e, %Y").to_string(),
                tag_thumbnails: vec![
                    TagThumbnailModel {
                        name: "Engineering".to_string(),
                        posts: vec![
                            PostThumbnailModel {
                                title: "How to build a bridge".to_string(),
                                subtitle: "A guide to building bridges".to_string(),
                                image_url: "https://picsum.photos/200/128".to_string(),
                                author: "Gregory Valentine".to_string(),
                            },
                            PostThumbnailModel {
                                title: "How to build a bridge".to_string(),
                                subtitle: "A guide to building bridges".to_string(),
                                image_url: "https://picsum.photos/200/128".to_string(),
                                author: "Gregory Valentine".to_string(),
                            },
                            PostThumbnailModel {
                                title: "How to build a bridge".to_string(),
                                subtitle: "A guide to building bridges".to_string(),
                                image_url: "https://picsum.photos/200/128".to_string(),
                                author: "Gregory Valentine".to_string(),
                            },
                        ],
                    },
                    TagThumbnailModel {
                        name: "Java".to_string(),
                        posts: vec![
                            PostThumbnailModel {
                                title: "How to brew coffee?".to_string(),
                                subtitle: "A step-by-step guide to brewing coffee".to_string(),
                                image_url: "https://picsum.photos/200/128".to_string(),
                                author: "Gregory Valentine".to_string(),
                            },
                            PostThumbnailModel {
                                title: "How to brew coffee?".to_string(),
                                subtitle: "A step-by-step guide to brewing coffee".to_string(),
                                image_url: "https://picsum.photos/200/128".to_string(),
                                author: "Gregory Valentine".to_string(),
                            },
                            PostThumbnailModel {
                                title: "How to brew coffee?".to_string(),
                                subtitle: "A step-by-step guide to brewing coffee".to_string(),
                                image_url: "https://picsum.photos/200/128".to_string(),
                                author: "Gregory Valentine".to_string(),
                            },
                        ],
                    },
                ],
                keyword_thumbnails: vec![],
                tags: vec![
                    TagModel {
                        name: "Engineering".to_string(),
                        url: "/tag/engineering".to_string(),
                    },
                    TagModel {
                        name: "Java".to_string(),
                        url: "/tag/java".to_string(),
                    },
                ],
                author: AuthorThumbnailModel {
                    name: "Gregory Valentine".to_string(),
                    image_url: "https://picsum.photos/128".to_string(),
                    profile_url: "/author/gregory-valentine".to_string(),
                },
            };
            render_template!(handlebars, "post", json!({"title": response_model.title, "model": response_model}))
        }
        EntityResult::Error(e) => HttpResponse::InternalServerError().body(format!("Error retrieving posts: [{:?}]", e)),
    }
}
