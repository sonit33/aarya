use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;

/// the following types are returned by the API service module (helps to remove sqlx dependency for WASM use)

#[derive(Validate, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct QuestionMutationModel {
    #[validate(range(min = 1, max = 99999))]
    pub question_id: u32,
    #[validate(range(min = 1, max = 99999))]
    pub course_id: u32,
    #[validate(range(min = 1, max = 99999))]
    pub chapter_id: u32,
    #[validate(length(min = 6, max = 64))]
    pub id_hash: String,
    #[validate(length(min = 8, max = 2048))]
    pub que_text: String,
    #[validate(length(min = 8, max = 2048))]
    pub que_description: String,
    pub choices: Value, // Assuming JSON structure is [{ "id": "", "text": "" }]
    pub answers: Value, // Assuming JSON structure is [{ "id": "" }]
    #[validate(length(min = 8, max = 2048))]
    pub ans_explanation: String,
    #[validate(length(min = 8, max = 1024))]
    pub ans_hint: String,
    #[validate(range(min = 1, max = 5))]
    pub que_difficulty: u8,
    #[validate(length(min = 8, max = 1024))]
    pub diff_reason: String,
}

#[derive(Validate, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct QuestionQueryModel {
    pub question_id: u32,
    pub course_id: u32,
    pub chapter_id: u32,
    pub id_hash: String,
    pub que_text: String,
    pub que_description: String,
    pub choices: String,
    pub que_difficulty: u8,
    pub diff_reason: String,
    pub ans_explanation: String,
    pub ans_hint: String,
    pub course_name: Option<String>,
    pub chapter_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TestQueryModel {
    pub test_id: u32,
    pub test_name: String,
    pub test_kind: i8,
    pub test_description: String,
    pub course_id: Option<u32>,
    pub course_name: Option<String>,
    pub chapter_id: Option<u32>,
    pub chapter_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ChapterQueryModel {
    pub chapter_id: u32,
    pub id_hash: String,
    pub course_id: u32,
    pub course_name: Option<String>,
    pub chapter_name: Option<String>,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CourseQueryModel {
    pub course_id: u32,
    pub name: String,
    pub id_hash: String,
    pub description: String,
}
