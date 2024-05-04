use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::result_types::{DatabaseErrorType, EntityResult, SuccessResultType};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TagEntity {
    tag_id: u32,
    tag_name: String,
    tag_description: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct AuthorEntity {
    author_id: u32,
    author_name: String,
    author_email: String,
    author_bio: String,
    author_photo_url: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PostEntity {
    post_id: u32,
    post_url: String,
    post_title: String,
    post_body: String,
    post_description: String,
    post_keywords: String,
    post_summary: String,
    post_timestamp: DateTime<Utc>,
    author_id: u32,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PostTagEntity {
    tag_id: u32,
    post_id: u32,
}

impl TagEntity {
    fn new() -> Self {
        TagEntity {
            tag_id: 0,
            tag_name: "not-set".to_string(),
            tag_description: "not-set".to_string(),
        }
    }

    pub async fn create(
        &self,
        pool: &MySqlPool,
    ) -> EntityResult<SuccessResultType> {
        let query = r#"
            INSERT INTO tags (tag_name, tag_description)
            VALUES (?, ?)
        "#;

        match sqlx::query(query).bind(&self.tag_name).bind(&self.tag_description).execute(pool).await {
            Ok(d) => EntityResult::Success(SuccessResultType::Created(self.tag_id as u64, d.rows_affected())),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Error creating tag".to_string(), e.to_string())),
        }
    }

    pub async fn find_tags(
        &self,
        pool: &MySqlPool,
    ) -> EntityResult<Vec<TagEntity>> {
        let query = r#"
            SELECT tag_id, tag_name, tag_description
            FROM tags 
            WHERE tag_id = ?
        "#;

        match sqlx::query_as::<_, TagEntity>(query).fetch_all(pool).await {
            Ok(tags) => EntityResult::Success(tags),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Error fetching tags".to_string(), e.to_string())),
        }
    }
}

impl Default for TagEntity {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthorEntity {
    fn new() -> Self {
        AuthorEntity {
            author_id: 0,
            author_name: "not-set".to_string(),
            author_email: "not-set".to_string(),
            author_bio: "not-set".to_string(),
            author_photo_url: "not-set".to_string(),
        }
    }

    pub async fn create(
        &self,
        pool: &MySqlPool,
    ) -> EntityResult<SuccessResultType> {
        let query = r#"
            INSERT INTO authors (author_name, author_email, author_bio, author_photo_url)
            VALUES (?, ?, ?, ?)
        "#;

        match sqlx::query(query)
            .bind(&self.author_name)
            .bind(&self.author_email)
            .bind(&self.author_bio)
            .bind(&self.author_photo_url)
            .execute(pool)
            .await
        {
            Ok(d) => EntityResult::Success(SuccessResultType::Created(self.author_id as u64, d.rows_affected())),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Error creating author".to_string(), e.to_string())),
        }
    }

    pub async fn find_authors(
        &self,
        pool: &MySqlPool,
    ) -> EntityResult<Vec<AuthorEntity>> {
        let query = r#"
            SELECT author_id, author_name, author_email, author_bio, author_photo_url
            FROM authors 
            WHERE author_id = ?
        "#;

        match sqlx::query_as::<_, AuthorEntity>(query).fetch_all(pool).await {
            Ok(authors) => EntityResult::Success(authors),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Error fetching authors".to_string(), e.to_string())),
        }
    }
}

impl Default for AuthorEntity {
    fn default() -> Self {
        Self::new()
    }
}

impl PostEntity {
    fn new() -> Self {
        PostEntity {
            post_id: 0,
            post_url: "not-set".to_string(),
            post_title: "not-set".to_string(),
            post_body: "not-set".to_string(),
            post_description: "not-set".to_string(),
            post_keywords: "not-set".to_string(),
            post_summary: "not-set".to_string(),
            post_timestamp: Utc::now(),
            author_id: 0,
        }
    }

    pub async fn create(
        &self,
        pool: &MySqlPool,
    ) -> EntityResult<SuccessResultType> {
        let query = r#"
            INSERT INTO posts (post_url, post_title, post_body, post_description, post_keywords, post_summary, post_timestamp, author_id)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#;

        match sqlx::query(query)
            .bind(&self.post_url)
            .bind(&self.post_title)
            .bind(&self.post_body)
            .bind(&self.post_description)
            .bind(&self.post_keywords)
            .bind(&self.post_summary)
            .bind(self.post_timestamp)
            .bind(self.author_id)
            .execute(pool)
            .await
        {
            Ok(d) => EntityResult::Success(SuccessResultType::Created(self.post_id as u64, d.rows_affected())),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Error creating post".to_string(), e.to_string())),
        }
    }

    pub async fn find_posts(
        &self,
        pool: &MySqlPool,
    ) -> EntityResult<Vec<PostEntity>> {
        let query = r#"
            SELECT post_id, post_url, post_title, post_body, post_description, post_keywords, post_summary, post_timestamp, author_id
            FROM posts 
            WHERE post_id = ?
        "#;

        match sqlx::query_as::<_, PostEntity>(query).fetch_all(pool).await {
            Ok(posts) => EntityResult::Success(posts),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Error fetching posts".to_string(), e.to_string())),
        }
    }
}

impl Default for PostEntity {
    fn default() -> Self {
        Self::new()
    }
}

impl PostTagEntity {
    fn new() -> Self {
        PostTagEntity { tag_id: 0, post_id: 0 }
    }

    pub async fn create(
        &self,
        pool: &MySqlPool,
    ) -> EntityResult<SuccessResultType> {
        let query = r#"
            INSERT INTO post_tags (tag_id, post_id)
            VALUES (?, ?)
        "#;

        match sqlx::query(query).bind(self.tag_id).bind(self.post_id).execute(pool).await {
            Ok(d) => EntityResult::Success(SuccessResultType::Created(self.tag_id as u64, d.rows_affected())),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Error creating post tag".to_string(), e.to_string())),
        }
    }

    pub async fn find_tag_posts(
        &self,
        pool: &MySqlPool,
    ) -> EntityResult<Vec<PostTagEntity>> {
        let query = r#"
            SELECT tag_id, post_id
            FROM post_tags 
            WHERE tag_id = ?
        "#;

        match sqlx::query_as::<_, PostTagEntity>(query).bind(self.tag_id).fetch_all(pool).await {
            Ok(post_tags) => EntityResult::Success(post_tags),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Error fetching post tags".to_string(), e.to_string())),
        }
    }

    pub async fn find_post_tags(
        &self,
        pool: &MySqlPool,
    ) -> EntityResult<Vec<PostTagEntity>> {
        let query = r#"
            SELECT tag_id, post_id
            FROM post_tags 
            WHERE post_id = ?
        "#;

        match sqlx::query_as::<_, PostTagEntity>(query).bind(self.post_id).fetch_all(pool).await {
            Ok(post_tags) => EntityResult::Success(post_tags),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Error fetching post tags".to_string(), e.to_string())),
        }
    }
}

impl Default for PostTagEntity {
    fn default() -> Self {
        Self::new()
    }
}
