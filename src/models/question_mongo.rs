use std::collections::HashMap;

use bson::Bson;
use serde::{Deserialize, Serialize};

use crate::traits::model_crud::ModelCrud;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum QuestionMode {
	Single,
	Multi,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct QuestionModel {
	pub model_id: String,
	pub question: String,
	pub choices: HashMap<String, String>,
	pub answers: Vec<String>,
	pub q_description: String,
	pub a_explanation: String,
	pub a_hint: String,
	pub tags: Vec<String>,
	pub difficulty: usize,
	pub d_reason: String,
	pub mode: QuestionMode,
}

impl Into<Bson> for QuestionModel {
	fn into(self) -> Bson {
		let document = bson::to_bson(&self)
			.expect("Failed to convert QuestionModel into Bson");
		match document {
			Bson::Document(doc) => Bson::Document(doc),
			_ => panic!("Expected Bson::Document"),
		}
	}
}

impl ModelCrud<QuestionModel> for QuestionModel {}