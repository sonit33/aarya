use std::fmt::Debug;

use bson::{doc, Document};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::errors::model_error::model_error::GeneralModelError;
use crate::utils::db::DbOps;

pub trait ModelCrud<T> where T: Debug + Serialize + DeserializeOwned + PartialEq + Clone + Send + Sync + Unpin {
	async fn all(db: &DbOps<T>) -> Result<Vec<T>, Box<dyn std::error::Error>> {
		match db.read_all().await {
			Ok(result) => {
				Ok(result)
			}
			Err(e) => Err(Box::new(GeneralModelError { reason: e.to_string() })),
		}
	}

	async fn find(db: &DbOps<T>, model_id: &String) -> Result<T, Box<dyn std::error::Error>> {
		match db.read_by_key("model_id".to_string(), model_id.to_string()).await {
			Ok(result) => {
				Ok(result)
			}
			Err(e) => Err(Box::new(GeneralModelError { reason: e.to_string() })),
		}
	}

	async fn some(db: &DbOps<T>, filter: Document) -> Result<Vec<T>, Box<dyn std::error::Error>> {
		match db.read_by_filter(filter).await {
			Ok(result) => {
				Ok(result)
			}
			Err(e) => Err(Box::new(GeneralModelError { reason: e.to_string() })),
		}
	}

	async fn update(db: &DbOps<T>, model_id: &String, changed_model: &T) -> Result<u64, Box<dyn std::error::Error>> {
		match db.update(doc! {"model_id": model_id}, doc! {"$set": bson::to_document(changed_model)?}).await {
			Ok(result) => {
				Ok(result)
			}
			Err(e) => Err(Box::new(GeneralModelError { reason: e.to_string() })),
		}
	}

	async fn delete(db: &DbOps<T>, model_id: &String) -> Result<u64, Box<dyn std::error::Error>> {
		match db.delete(model_id.to_string()).await {
			Ok(result) => {
				Ok(result)
			}
			Err(e) => Err(Box::new(GeneralModelError { reason: e.to_string() })),
		}
	}
	async fn save(db: &DbOps<T>, data: &T) -> Result<T, Box<dyn std::error::Error>> {
		match db.create(data).await {
			Ok(result) => {
				Ok(result)
			}
			Err(e) => Err(Box::new(GeneralModelError { reason: e.to_string() })),
		}
	}
}