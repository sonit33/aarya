use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum QuestionMode {
    Single,
    Multi,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct QuestionModel {
    pub q_id: String,
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