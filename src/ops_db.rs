use bson::{doc, Document};
use bson::oid::ObjectId;
use futures::TryStreamExt;
use mongodb::{Collection, Database, error::Result, results::InsertOneResult};
use mongodb::results::{DeleteResult, UpdateResult};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct DbOps<T>
    where
        T: Serialize + DeserializeOwned + Unpin + Send + Sync,
{
    collection: Collection<T>,
    db: Database,
}

impl<T> DbOps<T>
    where
        T: Serialize + DeserializeOwned + Unpin + Send + Sync,
{
    pub async fn new(connection_string: &str, db_name: &str, collection_name: &str) -> Self {
        let client_options = mongodb::options::ClientOptions::parse(connection_string).await.unwrap();
        let client = mongodb::Client::with_options(client_options).unwrap();
        let db = client.database(db_name);

        let collection = db.collection::<T>(collection_name);

        DbOps { db, collection }
    }
    pub async fn create(&self, data: T) -> Result<InsertOneResult> {
        self.collection.insert_one(data, None).await
    }

    pub async fn read_by(&self, id: ObjectId) -> Result<Option<T>> {
        let filter = doc! { "_id": id };
        self.collection.find_one(filter, None).await
    }
    pub async fn read(&self, filter: Document) -> Result<Vec<T>> {
        let mut cursor = self.collection.find(filter, None).await?;
        let mut results = Vec::new();

        while let Some(result) = cursor.try_next().await? {
            results.push(result);
        }

        Ok(results)
    }

    pub async fn update(&self, id: ObjectId, data: T) -> Result<UpdateResult> {
        let filter = doc! { "_id": id };
        let update = doc! { "$set": bson::to_document(&data)? };
        self.collection.update_one(filter, update, None).await
    }

    pub async fn delete(&self, id: ObjectId) -> Result<DeleteResult> {
        let filter = doc! { "_id": id };
        self.collection.delete_one(filter, None).await
    }
}
