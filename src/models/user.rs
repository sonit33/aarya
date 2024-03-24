use std::fmt::{Debug, Display};
use std::str::FromStr;

use bson::{Bson, doc};
use serde::{Deserialize, Serialize};

use crate::traits::model_crud::ModelCrud;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct UserModel {
	pub model_id: String,
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

impl ModelCrud<UserModel> for UserModel {}

