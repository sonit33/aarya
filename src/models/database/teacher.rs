use sqlx::{Error, MySqlPool};
use sqlx::mysql::MySqlQueryResult;
use time::OffsetDateTime;

#[derive(Debug, sqlx::FromRow)]
pub struct Teacher {
	pub teacher_id: i32,
	pub teacher_name: String,
	pub teacher_school: Option<String>,
	pub password: String,
	pub email_address: String,
	pub photo_url: Option<String>,
	pub blurb: Option<String>,
	pub education: Option<String>,
	pub skills: Option<String>,
	pub certifications: Option<String>,
	pub employed_at: Option<String>,
	pub availability_dow: i8,
	pub account_active: i8,
	pub venmo_handle: Option<String>,
	pub paypal_handle: Option<String>,
	pub added_timestamp: Option<OffsetDateTime>,
	pub updated_timestamp: Option<OffsetDateTime>,
	pub deleted_timestamp: Option<OffsetDateTime>,
	pub availability_tod: Option<i8>,
}

impl Teacher {
	pub async fn create(
		pool: &MySqlPool,
		teacher_name: &str,
		teacher_school: Option<&str>,
		password: &str,
		email_address: &str,
		photo_url: Option<&str>,
		blurb: Option<&str>,
		education: Option<&str>,
		skills: Option<&str>,
		certifications: Option<&str>,
		employed_at: Option<&str>,
		availability_dow: i8,
		account_active: i8,
		venmo_handle: Option<&str>,
		paypal_handle: Option<&str>,
		availability_tod: Option<i8>,
	) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"INSERT INTO teachers (teacher_name, teacher_school, password, email_address, photo_url, blurb, education, skills, certifications, employed_at, availability_dow, account_active, venmo_handle, paypal_handle, availability_tod) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
		)
			.bind(teacher_name)
			.bind(teacher_school)
			.bind(password)
			.bind(email_address)
			.bind(photo_url)
			.bind(blurb)
			.bind(education)
			.bind(skills)
			.bind(certifications)
			.bind(employed_at)
			.bind(availability_dow)
			.bind(account_active)
			.bind(venmo_handle)
			.bind(paypal_handle)
			.bind(availability_tod)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn read(pool: &MySqlPool, teacher_id: i32) -> Result<Option<Teacher>, Error> {
		let teacher = sqlx::query_as::<_, Teacher>(
			"SELECT * FROM teachers WHERE teacher_id = ?",
		)
			.bind(teacher_id)
			.fetch_optional(pool)
			.await;
		match teacher {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn update(
		pool: &MySqlPool,
		teacher_id: i32,
		teacher_name: &str,
		teacher_school: Option<&str>,
		password: &str,
		email_address: &str,
		photo_url: Option<&str>,
		blurb: Option<&str>,
		education: Option<&str>,
		skills: Option<&str>,
		certifications: Option<&str>,
		employed_at: Option<&str>,
		availability_dow: i8,
		account_active: i8,
		venmo_handle: Option<&str>,
		paypal_handle: Option<&str>,
		availability_tod: Option<i8>,
	) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"UPDATE teachers SET teacher_name = ?, teacher_school = ?, password = ?, email_address = ?, photo_url = ?, blurb = ?, education = ?, skills = ?, certifications = ?, employed_at = ?, availability_dow = ?, account_active = ?, venmo_handle = ?, paypal_handle = ?, availability_tod = ? WHERE teacher_id = ?",
		)
			.bind(teacher_name)
			.bind(teacher_school)
			.bind(password)
			.bind(email_address)
			.bind(photo_url)
			.bind(blurb)
			.bind(education)
			.bind(skills)
			.bind(certifications)
			.bind(employed_at)
			.bind(availability_dow)
			.bind(account_active)
			.bind(venmo_handle)
			.bind(paypal_handle)
			.bind(availability_tod)
			.bind(teacher_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn delete(pool: &MySqlPool, teacher_id: i32) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"UPDATE teachers SET deleted_timestamp = CURRENT_TIMESTAMP WHERE teacher_id = ?",
		)
			.bind(teacher_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}
}

