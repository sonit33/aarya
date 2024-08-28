use std::str::FromStr;

use futures::stream::TryStreamExt;
use log::debug;
use mongodb::{
    bson::{self, doc, oid::ObjectId, Bson},
    Client, Collection,
};

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    entities::result_types::{DatabaseErrorType, EntityResult, SuccessResultType},
    utils::environ::Environ,
};

pub struct Database;

impl Database {
    pub fn get_collection<T>(client: &Client, collection_name: &str) -> Collection<T> {
        client
            .database(Environ::default().db_name.as_str())
            .collection::<T>(collection_name)
    }

    pub async fn create<T>(collection: Collection<T>, entity: T) -> EntityResult<SuccessResultType>
    where
        T: Serialize + Unpin + Send + Sync,
    {
        match collection.insert_one(entity, None).await {
            Ok(result) => {
                EntityResult::Success(SuccessResultType::Created(result.inserted_id.to_string()))
            }
            Err(e) => EntityResult::Error(DatabaseErrorType::MutationError(
                format!("Error creating document in {}", collection.name()),
                e.to_string(),
            )),
        }
    }

    pub async fn find_all<T>(collection: Collection<T>) -> EntityResult<Vec<T>>
    where
        T: DeserializeOwned + Unpin + Send + Sync,
    {
        match collection.find(doc! {}, None).await {
            Ok(mut cursor) => {
                let mut entities = vec![];
                loop {
                    match cursor.try_next().await {
                        Ok(Some(entity)) => entities.push(entity),
                        Ok(None) => break,
                        Err(e) => {
                            return EntityResult::Error(DatabaseErrorType::QueryError(
                                format!("Error getting documents from {}", collection.name()),
                                e.to_string(),
                            ))
                        }
                    }
                }
                EntityResult::Success(entities)
            }
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError(
                format!("Error getting documents from {}", collection.name()),
                e.to_string(),
            )),
        }
    }

    pub async fn find_by<T, E>(collection: Collection<T>, key: String, value: E) -> EntityResult<T>
    where
        T: DeserializeOwned + Unpin + Send + Sync,
        E: Serialize + Into<Bson>,
    {
        match collection.find_one(doc! {key.clone(): value}, None).await {
            Ok(cursor) => match cursor {
                Some(r) => EntityResult::Success(r),
                None => EntityResult::Error(DatabaseErrorType::NotFound(
                    format!("Error finding document by {} in {}", key, collection.name()),
                    "Document not found".to_string(),
                )),
            },
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError(
                format!("Error getting documents from {}", collection.name()),
                e.to_string(),
            )),
        }
    }

    pub async fn find<T>(collection: Collection<T>, id: String) -> EntityResult<T>
    where
        T: DeserializeOwned + Unpin + Send + Sync,
    {
        let object_id = ObjectId::from_str(id.as_str()).unwrap();

        match collection.find_one(doc! {"_id": object_id}, None).await {
            Ok(cursor) => match cursor {
                Some(r) => EntityResult::Success(r),
                None => EntityResult::Error(DatabaseErrorType::NotFound(
                    format!("Error finding document {} in {}", id, collection.name()),
                    "Document not found".to_string(),
                )),
            },
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError(
                format!("Error getting documents from {}", collection.name()),
                e.to_string(),
            )),
        }
    }

    pub async fn update<T>(
        collection: Collection<T>,
        entity: T,
        id: String,
    ) -> EntityResult<SuccessResultType>
    where
        T: Serialize + Unpin + Send + Sync,
    {
        let object_id = ObjectId::from_str(id.as_str()).unwrap();

        let filter = doc! { "_id": object_id };
        let update = doc! { "$set": bson::to_bson(&entity).unwrap() };

        debug!("Filter: {:?}", filter);
        debug!("Update: {:?}", update);

        match collection.update_one(filter, update, None).await {
            Ok(result) => {
                if result.matched_count > 0 {
                    EntityResult::Success(SuccessResultType::Updated(id.clone()))
                } else {
                    EntityResult::Error(DatabaseErrorType::MutationError(
                        format!("No document found to update in {}", collection.name()),
                        "No matching document found".to_string(),
                    ))
                }
            }
            Err(e) => EntityResult::Error(DatabaseErrorType::MutationError(
                format!("Error updating document in {}", collection.name()),
                e.to_string(),
            )),
        }
    }
}
