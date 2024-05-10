use aarya_utils::hash_ops;
use chrono::{DateTime, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::result_types::{DatabaseErrorType, EntityResult, SuccessResultType};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TagEntity {
    pub tag_id: u32,
    pub tag_name: String,
    pub tag_description: String,
    pub tag_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TagQueryModel {
    pub tag_id: u32,
    pub tag_name: String,
    pub tag_description: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct AuthorEntity {
    pub author_id: Option<u32>,
    pub author_name: String,
    pub author_email: String,
    pub author_bio: String,
    pub author_photo_url: String,
    pub author_intro: String,
    pub author_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct AuthorQueryModel {
    pub author_id: Option<u32>,
    pub author_name: String,
    pub author_email: String,
    pub author_bio: String,
    pub author_photo_url: String,
    pub author_intro: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PostEntity {
    pub post_id: Option<u32>,
    pub post_url: String,
    pub post_title: String,
    pub post_body: String,
    pub post_description: String,
    pub post_keywords: String,
    pub post_tldr: String,
    pub post_subtitle: String,
    pub post_timestamp: NaiveDate,
    pub post_hero_image_url: String,
    pub post_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PostQueryModel {
    pub post_id: Option<u32>,
    pub post_url: String,
    pub post_title: String,
    pub post_body: String,
    pub post_description: String,
    pub post_keywords: String,
    pub post_tldr: String,
    pub post_subtitle: String,
    pub post_hero_image_url: String,
    pub post_timestamp: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PostTagEntity {
    pub tag_id: u32,
    pub post_id: u32,
    pub row_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PostAuthorEntity {
    pub post_id: u32,
    pub author_id: u32,
    pub row_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostManifestModel {
    pub post_file: String,
    pub post_title: String,
    pub tags: Vec<u32>,
    pub authors: Vec<u32>,
    pub display_date: NaiveDate,
    pub description: String,
    pub tldr: String,
    pub keywords: Vec<String>,
    pub subtitle: String,
    pub image_url: String,
}

impl TagEntity {
    pub fn new() -> Self {
        TagEntity {
            tag_id: 0,
            tag_name: "not-set".to_string(),
            tag_description: "not-set".to_string(),
            tag_hash: None,
        }
    }

    pub async fn create(
        &self,
        pool: &MySqlPool,
    ) -> EntityResult<SuccessResultType> {
        let tag_hash = hash_ops::string_hasher(format!("{}-{}", &self.tag_name, self.tag_id).as_str());
        let query = r#"
            INSERT INTO tags (tag_name, tag_description, tag_hash)
            VALUES (?, ?, ?)
        "#;

        match sqlx::query(query).bind(&self.tag_name).bind(&self.tag_description).bind(tag_hash).execute(pool).await {
            Ok(d) => EntityResult::Success(SuccessResultType::Created(self.tag_id as u64, d.rows_affected())),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Error creating tag".to_string(), e.to_string())),
        }
    }

    pub async fn find_tag(
        &self,
        pool: &MySqlPool,
    ) -> EntityResult<TagQueryModel> {
        let query = r#"
            SELECT tag_id, tag_name, tag_description
            FROM tags 
            WHERE tag_id = ?
        "#;

        match sqlx::query_as::<_, TagQueryModel>(query).bind(self.tag_id).fetch_one(pool).await {
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
            author_id: Some(0),
            author_name: "not-set".to_string(),
            author_email: "not-set".to_string(),
            author_bio: "not-set".to_string(),
            author_photo_url: "not-set".to_string(),
            author_intro: "not-set".to_string(),
            author_hash: None,
        }
    }

    pub async fn create(
        &self,
        pool: &MySqlPool,
    ) -> EntityResult<SuccessResultType> {
        let author_hash = hash_ops::string_hasher(format!("{}-{}", &self.author_name, self.author_email).as_str());
        let query = r#"
            INSERT INTO authors (author_name, author_email, author_bio, author_photo_url, author_intro, author_hash)
            VALUES (?, ?, ?, ?, ?, ?)
        "#;

        match sqlx::query(query)
            .bind(&self.author_name)
            .bind(&self.author_email)
            .bind(&self.author_bio)
            .bind(&self.author_photo_url)
            .bind(&self.author_intro)
            .bind(author_hash)
            .execute(pool)
            .await
        {
            Ok(d) => EntityResult::Success(SuccessResultType::Created(d.last_insert_id(), d.rows_affected())),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Error creating author".to_string(), e.to_string())),
        }
    }

    pub async fn find_authors(
        &self,
        pool: &MySqlPool,
    ) -> EntityResult<Vec<AuthorQueryModel>> {
        let query = r#"
            SELECT author_id, author_name, author_email, author_bio, author_photo_url, author_intro
            FROM authors 
            WHERE author_id = ?
        "#;

        match sqlx::query_as::<_, AuthorQueryModel>(query).fetch_all(pool).await {
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
            post_id: Some(0),
            post_url: "not-set".to_string(),
            post_title: "not-set".to_string(),
            post_body: "not-set".to_string(),
            post_description: "not-set".to_string(),
            post_keywords: "not-set".to_string(),
            post_tldr: "not-set".to_string(),
            post_timestamp: NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            post_hero_image_url: "not-set".to_string(),
            post_subtitle: "not-set".to_string(),
            post_hash: None,
        }
    }

    pub async fn create(
        &self,
        pool: &MySqlPool,
    ) -> EntityResult<SuccessResultType> {
        let post_hash = hash_ops::string_hasher(&self.post_url);

        let query = r#"
            INSERT INTO posts (post_url, post_title, post_body, post_description, post_keywords, post_tldr, post_subtitle, post_timestamp, post_hash, post_hero_image_url)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#;

        match sqlx::query(query)
            .bind(&self.post_url)
            .bind(&self.post_title)
            .bind(&self.post_body)
            .bind(&self.post_description)
            .bind(&self.post_keywords)
            .bind(&self.post_tldr)
            .bind(&self.post_subtitle)
            .bind(self.post_timestamp)
            .bind(post_hash)
            .bind(&self.post_hero_image_url)
            .execute(pool)
            .await
        {
            Ok(d) => EntityResult::Success(SuccessResultType::Created(d.last_insert_id(), d.rows_affected())),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Error creating post".to_string(), e.to_string())),
        }
    }

    pub async fn find_post(
        &self,
        pool: &MySqlPool,
    ) -> EntityResult<PostQueryModel> {
        let query = r#"
            SELECT 
                post_id, 
                post_url, 
                post_title, 
                post_body, 
                post_description, 
                post_keywords, 
                post_tldr,
                post_subtitle,
                post_hero_image_url,
                post_timestamp
            FROM posts
            WHERE post_hash = ?
        "#;

        match sqlx::query_as::<_, PostQueryModel>(query).bind(self.post_hash.clone().unwrap()).fetch_one(pool).await {
            Ok(posts) => EntityResult::Success(posts),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Error fetching posts".to_string(), e.to_string())),
        }
    }

    pub async fn find(
        &self,
        pool: &MySqlPool,
    ) -> EntityResult<Vec<PostQueryModel>> {
        let query = r#"
            SELECT 
                post_id, 
                post_url, 
                post_title, 
                post_body, 
                post_description, 
                post_keywords, 
                post_tldr, 
                post_subtitle,
                post_timestamp,
                post_hero_image_url
            FROM posts
        "#;

        match sqlx::query_as::<_, PostQueryModel>(query).fetch_all(pool).await {
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
    pub fn new() -> Self {
        PostTagEntity {
            tag_id: 0,
            post_id: 0,
            row_hash: None,
        }
    }

    pub async fn create(
        &self,
        pool: &MySqlPool,
    ) -> EntityResult<SuccessResultType> {
        let row_hash = hash_ops::string_hasher(format!("{}-{}", self.tag_id, self.post_id).as_str());
        let query = r#"
            INSERT INTO posts_tags (tag_id, post_id, row_hash)
            VALUES (?, ?, ?)
        "#;

        match sqlx::query(query).bind(self.tag_id).bind(self.post_id).bind(row_hash).execute(pool).await {
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
            FROM posts_tags 
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
            FROM posts_tags 
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

impl PostAuthorEntity {
    fn new() -> Self {
        PostAuthorEntity {
            post_id: 0,
            author_id: 0,
            row_hash: None,
        }
    }

    pub async fn create(
        &self,
        pool: &MySqlPool,
    ) -> EntityResult<SuccessResultType> {
        let row_hash = hash_ops::string_hasher(format!("{}-{}", self.post_id, self.author_id).as_str());
        let query = r#"
            INSERT INTO posts_authors (post_id, author_id, row_hash)
            VALUES (?, ?, ?)
        "#;

        match sqlx::query(query).bind(self.post_id).bind(self.author_id).bind(row_hash).execute(pool).await {
            Ok(d) => EntityResult::Success(SuccessResultType::Created(self.post_id as u64, d.rows_affected())),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Error creating post author".to_string(), e.to_string())),
        }
    }

    pub async fn find_post_authors(
        &self,
        pool: &MySqlPool,
    ) -> EntityResult<Vec<PostAuthorEntity>> {
        let query = r#"
            SELECT post_id, author_id
            FROM posts_authors 
            WHERE post_id = ?
        "#;

        match sqlx::query_as::<_, PostAuthorEntity>(query).bind(self.post_id).fetch_all(pool).await {
            Ok(post_authors) => EntityResult::Success(post_authors),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Error fetching post authors".to_string(), e.to_string())),
        }
    }

    pub async fn find_author_posts(
        &self,
        pool: &MySqlPool,
    ) -> EntityResult<Vec<PostAuthorEntity>> {
        let query = r#"
            SELECT post_id, author_id
            FROM posts_authors 
            WHERE author_id = ?
        "#;

        match sqlx::query_as::<_, PostAuthorEntity>(query).bind(self.author_id).fetch_all(pool).await {
            Ok(post_authors) => EntityResult::Success(post_authors),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Error fetching post authors".to_string(), e.to_string())),
        }
    }
}

impl Default for PostAuthorEntity {
    fn default() -> Self {
        Self::new()
    }
}
