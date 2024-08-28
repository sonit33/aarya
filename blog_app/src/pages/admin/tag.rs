use actix_web::{get, post, web, HttpResponse, Responder};
use handlebars::Handlebars;
use log::{debug, error, info};
use mongodb::Client;

use serde_json::json;

use crate::{
    entities::{blogs::TagEntity, result_types::EntityResult},
    models::{TagRequestModel, TagResponseModel},
    utils::{
        db_ops::{self, Database},
        file_ops,
        json_ops::{self, JsonOpsResult},
    },
};

#[get("/admin/tag")]
pub async fn get_create_tag(handlebars: web::Data<Handlebars<'_>>) -> impl Responder {
    render_template!(
        handlebars,
        "tag-create",
        json!({
            "title": "Add a new Tag",
            "schema": file_ops::read_file("./assets/schema/tag-schema.json").unwrap()
        })
    )
}

#[get("/admin/tags")]
pub async fn get_tag_list(
    handlebars: web::Data<Handlebars<'_>>,
    mongoc: web::Data<Client>,
) -> impl Responder {
    let collection = Database::get_collection(&mongoc, "tags");
    match db_ops::Database::find_all::<TagEntity>(collection).await {
        EntityResult::Success(r) => {
            debug!("{:?}", r);
            render_template!(
                handlebars,
                "tag-list",
                json!({
                    "title": "All Tags",
                    "tags": TagResponseModel::from_vec(r)
                })
            )
        }
        EntityResult::Error(e) => {
            error!("Failed to list tags: {:?}", e);
            HttpResponse::InternalServerError().body("Error listing tags")
        }
    }
}

#[get("/admin/tag/{id}")]
pub async fn get_edit_tag(
    handlebars: web::Data<Handlebars<'_>>,
    mongoc: web::Data<Client>,
    path: web::Path<String>,
) -> impl Responder {
    let tag_id = path.into_inner();
    let collection = Database::get_collection(&mongoc, "tags");
    match db_ops::Database::find::<TagEntity>(collection, tag_id).await {
        EntityResult::Success(r) => {
            debug!("{:?}", r);
            render_template!(
                handlebars,
                "tag-edit",
                json!({
                    "title": "Edit Tag",
                    "tag": TagResponseModel::from(r),
                    "schema": file_ops::read_file("./assets/schema/tag-schema.json").unwrap()
                })
            )
        }
        EntityResult::Error(e) => {
            error!("Failed to find tag: {:?}", e);
            HttpResponse::BadRequest().body("Error finding tag")
        }
    }
}

#[post("/admin/tag")]
pub async fn post_create_tag(
    model: web::Json<TagRequestModel>,
    mongoc: web::Data<Client>,
) -> impl Responder {
    debug!("{:?}", model);

    match json_ops::validate_json_text(
        "./assets/schema/tag-schema.json",
        serde_json::to_string(&model).unwrap().as_str(),
    ) {
        JsonOpsResult::Success(_) => {
            let collection = Database::get_collection(&mongoc, "tags");
            match db_ops::Database::create(collection, model.to()).await {
                EntityResult::Success(r) => {
                    info!("Tag created {:?}", r);
                    HttpResponse::Ok().body("Tag created")
                }
                EntityResult::Error(e) => {
                    error!("Failed to create tag: {:?}", e);
                    HttpResponse::BadRequest().body("Error creating tag")
                }
            }
        }
        JsonOpsResult::Error(e) => {
            error!("Failed to validate tag: {:?}", e);
            HttpResponse::BadRequest().body("Error validating tag")
        }
    }
}

#[post("/admin/tag/{id}")]
pub async fn post_edit_tag(
    mongoc: web::Data<Client>,
    path: web::Path<String>,
    model: web::Json<TagRequestModel>,
) -> impl Responder {
    let tag_id = path.into_inner();
    debug!("{:?}", model);

    match json_ops::validate_json_text(
        "./assets/schema/tag-schema.json",
        serde_json::to_string(&model).unwrap().as_str(),
    ) {
        JsonOpsResult::Success(_) => {
            let collection = Database::get_collection(&mongoc, "tags");
            match Database::update(collection, model.to(), tag_id).await {
                EntityResult::Success(r) => {
                    info!("Tag updated {:?}", r);
                    HttpResponse::Ok().body("Tag updated")
                }
                EntityResult::Error(e) => {
                    error!("Failed to update tag: {:?}", e);
                    HttpResponse::BadRequest().body("Error updating tag")
                }
            }
        }
        JsonOpsResult::Error(e) => {
            error!("Failed to validate tag: {:?}", e);
            HttpResponse::BadRequest().body("Error validating tag")
        }
    }
}
