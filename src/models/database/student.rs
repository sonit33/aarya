use sqlx::mysql::MySqlQueryResult;
use sqlx::{Error, MySqlPool};

use crate::utils::{hasher, random};

#[derive(Debug, sqlx::FromRow)]
pub struct Student {
    pub student_id: Option<u32>,
    pub id_hash: String,
    pub first_name: String,
    pub email_address: String,
    pub email_hash: String,
    pub pass_hash: Option<String>,
    pub over_13: bool,
    pub email_verified: bool,
    pub account_active: bool,
    pub added_timestamp: Option<time::OffsetDateTime>,
}

impl Student {
    pub fn new() -> Self {
        Student {
            student_id: Some(0),
            first_name: String::from("not-set"),
            email_address: String::from("not-set"),
            email_hash: String::from("not-set"),
            id_hash: String::from("not-set"),
            pass_hash: Some(String::from("not-set")),
            over_13: true,
            email_verified: false,
            account_active: false,
            added_timestamp: Some(time::OffsetDateTime::now_utc()),
        }
    }

    pub fn random(hash: &String) -> Self {
        Student {
            student_id: Some(0),
            first_name: hash.to_string(),
            email_address: format!("{}@email.com", hash.to_string()),
            email_hash: hash.to_string(),
            id_hash: hash.to_string(),
            pass_hash: Some(hash.to_string()),
            over_13: true,
            email_verified: false,
            account_active: false,
            added_timestamp: Some(time::OffsetDateTime::now_utc()),
        }
    }
}

impl Student {
    pub async fn create(&self, pool: &MySqlPool) -> Result<MySqlQueryResult, Error> {
        let id_hash = hasher::fast_hash(&random::generate_guid(8));
        let email_hash = hasher::cook_hash(&self.email_address).unwrap();
        let res = sqlx
            ::query(
                "INSERT INTO students (id_hash, first_name, email_address, email_hash, pass_hash, over_13, email_verified, account_active) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(id_hash)
            .bind(&self.first_name)
            .bind(&self.email_address)
            .bind(email_hash)
            .bind(&self.pass_hash)
            .bind(&self.over_13)
            .bind(&self.email_verified)
            .bind(&self.account_active)
            .execute(pool).await;
        match res {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    pub async fn read(&self, pool: &MySqlPool) -> Result<Option<Student>, Error> {
        let student = sqlx::query_as::<_, Student>("SELECT student_id, id_hash, first_name, email_address, pass_hash, email_hash, over_13, email_verified, account_active, added_timestamp FROM students WHERE student_id = ?")
            .bind(&self.student_id)
            .fetch_optional(pool)
            .await;
        match student {
            Ok(result) => match result {
                Some(r) => {
                    let mut s = r;
                    s.pass_hash = None;
                    Ok(Some(s))
                }
                None => Ok(None),
            },
            Err(e) => Err(e),
        }
    }

    pub async fn read_by_email(&self, pool: &MySqlPool) -> Result<Option<Student>, Error> {
        let student = sqlx::query_as::<_, Student>("SELECT student_id, id_hash, first_name, email_address, email_hash, pass_hash, over_13, email_verified, account_active, added_timestamp FROM students WHERE email_hash = ? or email_address = ?")
            .bind(&self.email_hash)
            .bind(&self.email_address)
            .fetch_optional(pool)
            .await;
        match student {
            Ok(result) => match result {
                Some(r) => {
                    let mut s = r;
                    s.pass_hash = None;
                    Ok(Some(s))
                }
                None => Ok(None),
            },
            Err(e) => Err(e),
        }
    }

    pub async fn read_password(&self, pool: &MySqlPool) -> Result<String, Error> {
        let student = sqlx::query_as::<_, Student>(
            "SELECT pass_hash FROM students WHERE email_hash = ? or email_address = ?",
        )
        .bind(&self.email_hash)
        .bind(&self.email_address)
        .fetch_optional(pool)
        .await;
        match student {
            Ok(result) => match result {
                Some(r) => Ok(r.pass_hash.unwrap()),
                None => Ok(String::from("not-found")),
            },
            Err(e) => Err(e),
        }
    }

    pub async fn update(&self, pool: &MySqlPool) -> Result<MySqlQueryResult, Error> {
        let res = sqlx::query("UPDATE students SET first_name = ? WHERE student_id = ?")
            .bind(&self.first_name)
            .bind(&self.student_id)
            .execute(pool)
            .await;
        match res {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    pub async fn update_account(&self, pool: &MySqlPool) -> Result<MySqlQueryResult, Error> {
        let res = sqlx::query(
            "UPDATE students SET email_verified = ?, account_active = ? WHERE student_id = ?",
        )
        .bind(&self.email_verified)
        .bind(&self.account_active)
        .bind(&self.student_id)
        .execute(pool)
        .await;
        match res {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    pub async fn update_email_hash(&self, pool: &MySqlPool) -> Result<MySqlQueryResult, Error> {
        let email_hash = hasher::fast_hash(&self.email_address);
        let res = sqlx::query("UPDATE students SET email_hash = ? WHERE student_id = ?")
            .bind(email_hash)
            .execute(pool)
            .await;
        match res {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    pub async fn update_pass_hash(&self, pool: &MySqlPool) -> Result<MySqlQueryResult, Error> {
        let res = sqlx::query("UPDATE students SET pass_hash = ? WHERE student_id = ?")
            .bind(&self.pass_hash)
            .bind(&self.student_id)
            .execute(pool)
            .await;
        match res {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    pub async fn delete(&self, pool: &MySqlPool) -> Result<MySqlQueryResult, Error> {
        let res = sqlx::query("DELETE from students where student_id = ?")
            .bind(&self.student_id)
            .execute(pool)
            .await;
        match res {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }
}
