use serde::{ Deserialize, Serialize };
use serde_json::Value;
use validator::Validate;

#[derive(Validate, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct QuestionModel {
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
    pub difficulty: u8,
    #[validate(length(min = 8, max = 1024))]
    pub diff_reason: String,
}
