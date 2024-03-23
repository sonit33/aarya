#![allow(dead_code)]

use std::error::Error;
use std::fmt::{Display, Formatter};

use bson::{doc, Document};
use futures::TryStreamExt;
use mongodb::{Collection, Database};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::models::user::UserModel;
use crate::utils::environ::Environ;

pub struct DbOps<T>
    where
        T: Serialize + DeserializeOwned + Unpin + Send + Sync,
{
    pub collection: Collection<T>,
    pub db: Database,
}

impl<T> DbOps<T>
    where
        T: Serialize + DeserializeOwned + Unpin + Send + Sync,
{
    pub async fn new(connection_string: String, db_name: String, collection_name: String) -> Result<DbOps<T>, Box<dyn Error>> {
        // let client_options = mongodb::options::ClientOptions::parse(connection_string).await.unwrap();
        match mongodb::options::ClientOptions::parse(connection_string.clone()).await {
            Ok(client_options) => {
                match mongodb::Client::with_options(client_options) {
                    Ok(client) => {
                        let db = client.database(db_name.as_str());
                        let collection = db.collection::<T>(collection_name.as_str());

                        Ok(DbOps { db, collection })
                    }
                    Err(e) => Err(Box::new(DatabaseConnectionFailedError { connection_string: connection_string.to_string(), error: e.to_string() }))
                }
            }
            Err(e) => Err(Box::new(DatabaseConnectionFailedError { connection_string: connection_string.to_string(), error: e.to_string() }))
        }
    }

    pub async fn get_db(db_name: String, collection_name: String) -> Result<DbOps<UserModel>, Box<dyn std::error::Error>> {
        let environ = Environ::default();
        match DbOps::new(environ.mongo_connection_string, db_name, collection_name).await {
            Ok(db_user) => Ok(db_user), // On success, return the DbOps instance
            Err(e) => Err(e),           // On failure, forward the error
        }
    }
    pub async fn create(&self, data: T) -> Result<T, Box<dyn Error>> {
        match self.collection.insert_one(&data, None).await {
            Ok(result) => { Ok(data) }
            Err(e) => Err(Box::new(RecordNotCreatedError { id: "not-set".to_string(), reason: e.to_string() }))
        }
    }

    pub async fn read_by_key(&self, key: String, value: String) -> Result<T, Box<dyn Error>> {
        let filter = doc! { key: value.clone() };

        match self.collection.find_one(filter, None).await {
            Ok(result) => {
                match result {
                    Some(document) => Ok(document),
                    None => Err(Box::new(RecordNotFoundError { id: value, reason: "no-result".to_string() })),
                }
            }
            Err(e) => Err(Box::new(RecordNotFoundError { id: value, reason: e.to_string() })),
        }
    }
    pub async fn read_by_filter(&self, filter: Document) -> Result<Vec<T>, Box<dyn Error>> {
        match self.collection.find(filter, None).await {
            Ok(mut cursor) => {
                let mut results = Vec::new();
                while let Some(result) = cursor.try_next().await? {
                    results.push(result);
                }

                Ok(results)
            }
            Err(e) => Err(Box::new(RecordNotFoundError { id: "not-set".to_string(), reason: e.to_string() })),
        }
    }

    pub async fn read_all(&self) -> Result<Vec<T>, Box<dyn Error>> {
        match self.collection.find(doc! {}, None).await {
            Ok(mut cursor) => {
                let mut results = Vec::new();

                while let Some(result) = cursor.try_next().await? {
                    results.push(result);
                }

                Ok(results)
            }
            Err(e) => Err(Box::new(RecordNotFoundError { id: "not-set".to_string(), reason: e.to_string() })),
        }
    }

    pub async fn update(&self, id: String, data: T) -> Result<String, Box<dyn Error>> {
        let filter = doc! { "_id": id.clone() };
        let update = doc! { "$set": bson::to_document(&data)? };
        match self.collection.update_one(filter, update, None).await {
            Ok(result) => {
                Ok(result.upserted_id.unwrap().to_string())
            }
            Err(e) => Err(Box::new(RecordNotUpdatedError { id, reason: e.to_string() })),
        }
    }

    pub async fn delete(&self, id: String) -> Result<u64, Box<dyn Error>> {
        let filter = doc! { "_id": id.clone() };
        match self.collection.delete_one(filter, None).await {
            Ok(result) => {
                Ok(result.deleted_count)
            }
            Err(e) => Err(Box::new(RecordNotDeletedError { id, reason: e.to_string() })),
        }
    }
}

#[derive(Debug)]
pub struct DatabaseConnectionFailedError {
    pub connection_string: String,
    pub error: String,
}

impl Display for DatabaseConnectionFailedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Database connection failed for {} due to {}", self.connection_string, self.error)
    }
}

impl Error for DatabaseConnectionFailedError {}

#[derive(Debug)]
pub struct RecordNotFoundError {
    pub id: String,
    pub reason: String,
}

impl Display for RecordNotFoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Document with ID {} not found", self.id)
    }
}

impl Error for RecordNotFoundError {}

#[derive(Debug)]
pub struct RecordNotCreatedError {
    pub id: String,
    pub reason: String,
}

impl Display for RecordNotCreatedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Document with ID {} not created", self.id)
    }
}

impl Error for RecordNotCreatedError {}

#[derive(Debug)]
pub struct RecordNotUpdatedError {
    pub id: String,
    pub reason: String,
}

impl Display for RecordNotUpdatedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Document with ID {} not updated", self.id)
    }
}

impl Error for RecordNotUpdatedError {}

#[derive(Debug)]
pub struct RecordNotDeletedError {
    pub id: String,
    pub reason: String,
}

impl Display for RecordNotDeletedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Document with ID {} not deleted", self.id)
    }
}

impl Error for RecordNotDeletedError {}

