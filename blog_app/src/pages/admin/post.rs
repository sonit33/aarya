use actix_web::{get, post, web, HttpResponse, Responder};
use handlebars::Handlebars;
use log::{debug, error, info};
use mongodb::Client;

use serde_json::json;

use crate::{
    entities::{
        blogs::{AuthorEntity, PostEntity, TagEntity},
        result_types::EntityResult,
    },
    models::{AuthorResponseModel, PostRequestModel, PostResponseModel, TagResponseModel},
    utils::{
        date_ops,
        db_ops::Database,
        file_ops,
        json_ops::{self, JsonOpsResult},
    },
};

#[get("/admin/post")]
pub async fn get_create_post(
    handlebars: web::Data<Handlebars<'_>>,
    mongoc: web::Data<Client>,
) -> impl Responder {
    let authors_collection = Database::get_collection(&mongoc, "authors");
    let authors = match Database::find_all::<AuthorEntity>(authors_collection).await {
        EntityResult::Success(r) => r,
        EntityResult::Error(e) => {
            error!("Failed to find authors: {:?}", e);
            return HttpResponse::InternalServerError().body("Error finding authors");
        }
    };

    let tags_collection = Database::get_collection(&mongoc, "tags");
    let tags = match Database::find_all::<TagEntity>(tags_collection).await {
        EntityResult::Success(r) => r,
        EntityResult::Error(e) => {
            error!("Failed to find tags: {:?}", e);
            return HttpResponse::InternalServerError().body("Error finding tags");
        }
    };
    render_template!(
        handlebars,
        "post-create",
        json!({
            "title": "Add a new Post",
            "timestamp": date_ops::to_input_date(),
            "authors": AuthorResponseModel::from_vec(authors),
            "tags": TagResponseModel::from_vec(tags),
            "schema": file_ops::read_file("./assets/schema/post-schema.json").unwrap()
        })
    )
}

#[post("/admin/post")]
pub async fn post_create_post(
    model: web::Json<PostRequestModel>,
    mongoc: web::Data<Client>,
) -> impl Responder {
    debug!("{:?}", model);

    match json_ops::validate_json_text(
        "./assets/schema/post-schema.json",
        serde_json::to_string(&model).unwrap().as_str(),
    ) {
        JsonOpsResult::Success(_) => {
            let collection = Database::get_collection(&mongoc, "posts");
            match Database::create(collection, model.to()).await {
                EntityResult::Success(r) => {
                    info!("Post created {:?}", r);
                    HttpResponse::Ok().body("Post created")
                }
                EntityResult::Error(e) => {
                    error!("Failed to create author: {:?}", e);
                    HttpResponse::BadRequest().body("Error creating post")
                }
            }
        }
        JsonOpsResult::Error(e) => {
            error!("Failed to validate post: {:?}", e);
            HttpResponse::BadRequest().body("Error validating post")
        }
    }
}

#[get("/admin/posts")]
pub async fn get_post_list(
    handlebars: web::Data<Handlebars<'_>>,
    mongoc: web::Data<Client>,
) -> impl Responder {
    // get all posts
    let collection = Database::get_collection(&mongoc, "posts");
    let posts = match Database::find_all::<PostEntity>(collection).await {
        EntityResult::Success(r) => r,
        EntityResult::Error(e) => {
            error!("Failed to find posts: {:?}", e);
            return HttpResponse::InternalServerError().body("Error finding post");
        }
    };

    let authors_collection = Database::get_collection(&mongoc, "authors");
    let authors = match Database::find_all::<AuthorEntity>(authors_collection).await {
        EntityResult::Success(r) => r,
        EntityResult::Error(e) => {
            error!("Failed to find authors: {:?}", e);
            return HttpResponse::InternalServerError().body("Error finding authors");
        }
    };

    let tags_collection = Database::get_collection(&mongoc, "tags");
    let tags = match Database::find_all::<TagEntity>(tags_collection).await {
        EntityResult::Success(r) => r,
        EntityResult::Error(e) => {
            error!("Failed to find tags: {:?}", e);
            return HttpResponse::InternalServerError().body("Error finding tags");
        }
    };

    let result = PostResponseModel::all(posts, authors, tags);

    render_template!(
        handlebars,
        "post-list",
        json!({
            "title": "All Posts",
            "posts": result
        })
    )
}

#[get("/admin/post/{id}")]
pub async fn get_edit_post(
    handlebars: web::Data<Handlebars<'_>>,
    mongoc: web::Data<Client>,
    path: web::Path<String>,
) -> impl Responder {
    let post_id = path.into_inner();
    let authors_collection = Database::get_collection(&mongoc, "authors");
    let authors = match Database::find_all::<AuthorEntity>(authors_collection).await {
        EntityResult::Success(r) => r,
        EntityResult::Error(e) => {
            error!("Failed to find authors: {:?}", e);
            return HttpResponse::InternalServerError().body("Error finding authors");
        }
    };

    let tags_collection = Database::get_collection(&mongoc, "tags");
    let tags = match Database::find_all::<TagEntity>(tags_collection).await {
        EntityResult::Success(r) => r,
        EntityResult::Error(e) => {
            error!("Failed to find tags: {:?}", e);
            return HttpResponse::InternalServerError().body("Error finding tags");
        }
    };

    let collection = Database::get_collection(&mongoc, "posts");
    match Database::find::<PostEntity>(collection, post_id).await {
        EntityResult::Success(r) => {
            debug!("{:?}", r);
            let post = PostResponseModel::combine(r.clone(), authors.clone(), tags.clone());
            render_template!(
                handlebars,
                "post-edit",
                json!({
                    "title": "Edit Post",
                    "post": post,
                    "body_markdown": format!("/assets/markdowns/{}.md",post.body),
                    "authors": AuthorResponseModel::from_vec(authors.clone()),
                    "tags": TagResponseModel::from_vec(tags.clone()),
                    "timestamp": date_ops::to_input_date(),
                    "published_timestamp": date_ops::to_input_date_from(r.publish_date),
                    "schema": file_ops::read_file("./assets/schema/post-schema.json").unwrap()
                })
            )
        }
        EntityResult::Error(e) => {
            error!("Failed to find post: {:?}", e);
            HttpResponse::BadRequest().body("Error finding post")
        }
    }
}

#[post("/admin/post/{id}")]
pub async fn post_edit_post(
    mongoc: web::Data<Client>,
    path: web::Path<String>,
    model: web::Json<PostRequestModel>,
) -> impl Responder {
    let post_id = path.into_inner();
    debug!("{:?}", model);

    match json_ops::validate_json_text(
        "./assets/schema/post-schema.json",
        serde_json::to_string(&model).unwrap().as_str(),
    ) {
        JsonOpsResult::Success(_) => {
            let collection = Database::get_collection(&mongoc, "posts");
            match Database::update(collection, model.to(), post_id).await {
                EntityResult::Success(r) => {
                    info!("Post updated {:?}", r);
                    HttpResponse::Ok().body("Post updated")
                }
                EntityResult::Error(e) => {
                    error!("Failed to update post: {:?}", e);
                    HttpResponse::BadRequest().body("Error updating post")
                }
            }
        }
        JsonOpsResult::Error(e) => {
            error!("Failed to validate post: {:?}", e);
            HttpResponse::BadRequest().body("Error validating post")
        }
    }
}
