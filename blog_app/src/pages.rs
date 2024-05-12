use aarya_utils::hash_ops::string_hasher;
use actix_web::{get, web, HttpResponse, Responder};
use handlebars::Handlebars;
use models::{
    blogs::{PostEntity, PostQueryModel},
    result_types::EntityResult,
};
use pulldown_cmark::{html, Parser};
use serde_json::json;
use sqlx::MySqlPool;

use crate::models::{
    AuthorThumbnailModel, IndexPostImageResponseModel, IndexPostTextResponseModel, IndexResponseModel, KeywordThumbnailModel, PostResponseModel, PostThumbnailModel, TagModel, TagThumbnailModel,
};

fn generate_random_post_image() -> IndexPostImageResponseModel {
    IndexPostImageResponseModel {
        author: AuthorThumbnailModel {
            name: format!("Author {}", rand::random::<u8>()),
            image_url: String::from("https://picsum.photos/128"),
            profile_url: format!("/author/author-{}", rand::random::<u8>()),
        },
        featured_image: String::from("https://picsum.photos/720/360"),
        thumbnail_image: String::from("https://picsum.photos/380/190"),
        tag: TagModel {
            name: format!("Category {}", rand::random::<u8>()),
            url: format!("/tag/category-{}", rand::random::<u8>()),
        },
        title: format!("Post Title {}", rand::random::<u8>()),
        subtitle: format!("Subtitle {}", rand::random::<u8>()),
        is_featured: rand::random(),
    }
}

fn generate_random_post_text() -> IndexPostTextResponseModel {
    IndexPostTextResponseModel {
        author: AuthorThumbnailModel {
            name: format!("Author {}", rand::random::<u8>()),
            image_url: String::from("https://picsum.photos/128"),
            profile_url: format!("/author/author-{}", rand::random::<u8>()),
        },
        tag: TagModel {
            name: format!("Category {}", rand::random::<u8>()),
            url: format!("/tag/category-{}", rand::random::<u8>()),
        },
        title: format!("Post Title {}", rand::random::<u8>()),
        subtitle: format!("Subtitle {}", rand::random::<u8>()),
    }
}

#[get("/")]
pub async fn home_page(
    handlebars: web::Data<Handlebars<'_>>,
    _pool: web::Data<MySqlPool>,
) -> impl Responder {
    let hero_posts = (0..4).map(|_| generate_random_post_image()).collect();
    let featured_posts = (0..4).map(|_| generate_random_post_image()).collect();
    let latest_posts = (0..4).map(|_| generate_random_post_text()).collect();
    let posts_by_tags = (0..4).map(|_| generate_random_post_image()).collect();
    let trending_posts = (0..4).map(|_| generate_random_post_text()).collect();

    let response = IndexResponseModel {
        title: "The Aarya AI Blog".to_string(),
        hero_posts,
        featured_posts,
        latest_posts,
        posts_by_tags,
        trending_posts,
    };
    render_template!(handlebars, "index", json!({"title": response.title, "model": response}))
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
                keywords: post.post_keywords,
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
                keyword_thumbnails: vec![
                    KeywordThumbnailModel {
                        name: "data structures".to_string(),
                        posts: vec![
                            PostThumbnailModel {
                                title: "Making sense of the AP Computer Science Principles exam format".to_string(),
                                subtitle: "Linking lsists are fun".to_string(),
                                image_url: "https://picsum.photos/200/128".to_string(),
                                author: "Augustus Albequerque".to_string(),
                            },
                            PostThumbnailModel {
                                title: "Binary Search Trees".to_string(),
                                subtitle: "Let's shake a few trees".to_string(),
                                image_url: "https://picsum.photos/200/128".to_string(),
                                author: "Augustus Albequerque".to_string(),
                            },
                        ],
                    },
                    KeywordThumbnailModel {
                        name: "algorithms".to_string(),
                        posts: vec![
                            PostThumbnailModel {
                                title: "Merge Sort, the simple way".to_string(),
                                subtitle: "There is no way Merge sorts are simple".to_string(),
                                image_url: "https://picsum.photos/200/128".to_string(),
                                author: "Augustus Albequerque".to_string(),
                            },
                            PostThumbnailModel {
                                title: "Binary Search".to_string(),
                                subtitle: "This one is simple, indeed".to_string(),
                                image_url: "https://picsum.photos/200/128".to_string(),
                                author: "Augustus Albequerque".to_string(),
                            },
                        ],
                    },
                ],
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
