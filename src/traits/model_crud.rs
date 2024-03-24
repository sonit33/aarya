use bson::{doc, Document};

use crate::errors::model_error::model_error::GeneralModelError;
use crate::models::user::UserModel;
use crate::utils::db::DbOps;

pub trait ModelCrud {
    async fn all(db: &DbOps<UserModel>) -> Result<Vec<UserModel>, Box<dyn std::error::Error>> {
        match db.read_all().await {
            Ok(result) => {
                Ok(result)
            }
            Err(e) => Err(Box::new(GeneralModelError { reason: e.to_string() })),
        }
    }

    async fn find(db: &DbOps<UserModel>, user_id: &String) -> Result<UserModel, Box<dyn std::error::Error>> {
        match db.read_by_key("user_id".to_string(), user_id.to_string()).await {
            Ok(result) => {
                Ok(result)
            }
            Err(e) => Err(Box::new(GeneralModelError { reason: e.to_string() })),
        }
    }

    async fn some(db: &DbOps<UserModel>, filter: Document) -> Result<Vec<UserModel>, Box<dyn std::error::Error>> {
        match db.read_by_filter(filter).await {
            Ok(result) => {
                Ok(result)
            }
            Err(e) => Err(Box::new(GeneralModelError { reason: e.to_string() })),
        }
    }

    async fn update(db: &DbOps<UserModel>, user_id: &String, changed_model: &UserModel) -> Result<u64, Box<dyn std::error::Error>> {
        match db.update(doc! {"user_id": user_id}, doc! {"$set": bson::to_document(changed_model)?}).await {
            Ok(result) => {
                Ok(result)
            }
            Err(e) => Err(Box::new(GeneralModelError { reason: e.to_string() })),
        }
    }

    async fn delete(db: &DbOps<UserModel>, user_id: &String) -> Result<u64, Box<dyn std::error::Error>> {
        match db.delete(user_id.to_string()).await {
            Ok(result) => {
                Ok(result)
            }
            Err(e) => Err(Box::new(GeneralModelError { reason: e.to_string() })),
        }
    }
    async fn save(db: &DbOps<UserModel>, data: &UserModel) -> Result<UserModel, Box<dyn std::error::Error>> {
        match db.create(data).await {
            Ok(result) => {
                Ok(result)
            }
            Err(e) => Err(Box::new(GeneralModelError { reason: e.to_string() })),
        }
    }
}