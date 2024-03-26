use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ActionType {
    Redirect,
    Resolve,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseAction {
    pub action_type: ActionType,
    pub arg: String,
}

#[derive(Serialize, Deserialize)]
pub struct DefaultResponseModel<T: Serialize> {
    pub message: String,
    pub payload: T,
    pub action: ResponseAction,
}