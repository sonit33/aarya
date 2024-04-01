use sqlx::{ Error, MySqlPool };
use sqlx::mysql::MySqlQueryResult;

use crate::utils::{ hasher, random };

#[derive(Debug, sqlx::FromRow)]
pub struct Student {
    pub student_id: Option<i32>,
    pub id_hash: String,
    pub first_name: String,
    pub email_address: String,
    pub email_hash: String,
    pub password: String,
    pub over_13: bool,
    pub email_verified: bool,
    pub account_active: bool,
    pub added_timestamp: Option<time::OffsetDateTime>,
    pub updated_timestamp: Option<time::OffsetDateTime>,
    pub deleted_timestamp: Option<time::OffsetDateTime>,
}

impl Student {
    fn random(hash: &str) -> Self {
        Student {
            student_id: Some(0),
            first_name: hash.to_string(),
            email_address: format!("{}@email.com", hash),
            email_hash: hasher::fast_hash(format!("{}@email.com", hash).as_str()),
            id_hash: hasher::fast_hash(&hash),
            password: hash.to_string(),
            over_13: true,
            email_verified: false,
            account_active: false,
            added_timestamp: Some(time::OffsetDateTime::now_utc()),
            updated_timestamp: Some(time::OffsetDateTime::now_utc()),
            deleted_timestamp: Some(time::OffsetDateTime::now_utc()),
        }
    }
}

impl Student {
    pub async fn create(
        pool: &MySqlPool,
        first_name: &str,
        email_address: &str,
        password: &str,
        over_13: bool,
        email_verified: bool,
        account_active: bool
    ) -> Result<MySqlQueryResult, Error> {
        let id_hash = hasher::fast_hash(&random::generate_guid(8));
        let email_hash = hasher::cook_hash(email_address).unwrap();
        let res = sqlx
            ::query(
                "INSERT INTO students (id_hash, first_name, email_address, email_hash, password, over_13, email_verified, account_active) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(id_hash)
            .bind(first_name)
            .bind(email_address)
            .bind(email_hash)
            .bind(password)
            .bind(over_13)
            .bind(email_verified)
            .bind(account_active)
            .execute(pool).await;
        match res {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    pub async fn read(pool: &MySqlPool, student_id: i32) -> Result<Option<Student>, Error> {
        let student = sqlx
            ::query_as::<_, Student>("SELECT * FROM students WHERE student_id = ?")
            .bind(student_id)
            .fetch_optional(pool).await;
        match student {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    pub async fn read_by_email(
        pool: &MySqlPool,
        email_address: &str
    ) -> Result<Option<Student>, Error> {
        let student = sqlx
            ::query_as::<_, Student>("SELECT * FROM students WHERE email_address = ?")
            .bind(email_address)
            .fetch_optional(pool).await;
        match student {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    pub async fn read_by_email_hash(
        pool: &MySqlPool,
        email_hash: &str
    ) -> Result<Option<Student>, Error> {
        let student = sqlx
            ::query_as::<_, Student>("SELECT * FROM students WHERE email_hash = ?")
            .bind(email_hash)
            .fetch_optional(pool).await;
        match student {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    pub async fn update(&self, pool: &MySqlPool) -> Result<MySqlQueryResult, Error> {
        let email_hash = hasher::fast_hash(&self.email_address);
        let res = sqlx
            ::query(
                "UPDATE students SET first_name = ?, email_address = ?, email_hash = ?, password = ?, over_13 = ?, email_verified = ?, account_active = ? WHERE student_id = ?"
            )
            .bind(&self.first_name)
            .bind(&self.email_address)
            .bind(email_hash)
            .bind(&self.password)
            .bind(&self.over_13)
            .bind(&self.email_verified)
            .bind(&self.account_active)
            .bind(&self.student_id)
            .execute(pool).await;
        match res {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    pub async fn delete(pool: &MySqlPool, student_id: i32) -> Result<MySqlQueryResult, Error> {
        let res = sqlx
            ::query(
                "UPDATE students SET deleted_timestamp = CURRENT_TIMESTAMP WHERE student_id = ?"
            )
            .bind(student_id)
            .execute(pool).await;
        match res {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }
}
