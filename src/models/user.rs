use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use bson::{Bson, doc, Document};
use serde::{Deserialize, Serialize};

use crate::utils::db::DbOps;

#[derive(Debug)]
pub struct UserModelError {
    pub reason: String,
}

impl Display for UserModelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "User model failed due to {}", self.reason)
    }
}

impl Error for UserModelError {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct UserModel {
    pub user_id: String,
    pub display_name: String,
    pub email_address: String,
    pub password: String,
    pub over_13: bool,
    pub email_verified: bool,
    pub account_active: bool,
    pub mark_deleted: bool,
}

impl Into<Bson> for UserModel {
    fn into(self) -> Bson {
        let document = bson::to_bson(&self)
            .expect("Failed to convert UserModel into Bson");
        match document {
            Bson::Document(doc) => Bson::Document(doc),
            _ => panic!("Expected Bson::Document"),
        }
    }
}

impl UserModel {
    pub async fn all(db: &DbOps<UserModel>) -> Result<Vec<UserModel>, Box<dyn std::error::Error>> {
        match db.read_all().await {
            Ok(result) => {
                Ok(result)
            }
            Err(e) => Err(Box::new(UserModelError { reason: e.to_string() })),
        }
    }

    pub async fn find(db: &DbOps<UserModel>, user_id: &String) -> Result<UserModel, Box<dyn std::error::Error>> {
        match db.read_by_key("user_id".to_string(), user_id.to_string()).await {
            Ok(result) => {
                Ok(result)
            }
            Err(e) => Err(Box::new(UserModelError { reason: e.to_string() })),
        }
    }

    pub async fn some(db: &DbOps<UserModel>, filter: Document) -> Result<Vec<UserModel>, Box<dyn std::error::Error>> {
        match db.read_by_filter(filter).await {
            Ok(result) => {
                Ok(result)
            }
            Err(e) => Err(Box::new(UserModelError { reason: e.to_string() })),
        }
    }

    pub async fn update(db: &DbOps<UserModel>, user_id: &String, changed_model: &UserModel) -> Result<u64, Box<dyn std::error::Error>> {
        match db.update(doc! {"user_id": user_id}, doc! {"$set": bson::to_document(changed_model)?}).await {
            Ok(result) => {
                Ok(result)
            }
            Err(e) => Err(Box::new(UserModelError { reason: e.to_string() })),
        }
    }

    pub async fn delete(db: &DbOps<UserModel>, user_id: &String) -> Result<u64, Box<dyn std::error::Error>> {
        match db.delete(user_id.to_string()).await {
            Ok(result) => {
                Ok(result)
            }
            Err(e) => Err(Box::new(UserModelError { reason: e.to_string() })),
        }
    }
    pub async fn save(&self, db: &DbOps<UserModel>) -> Result<UserModel, Box<dyn std::error::Error>> {
        match db.create(self.to_owned()).await {
            Ok(result) => {
                Ok(result)
            }
            Err(e) => Err(Box::new(UserModelError { reason: e.to_string() })),
        }
    }
}

