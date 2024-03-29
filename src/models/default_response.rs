use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ActionType {
	Redirect,
	HandleError,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseAction {
	pub action_type: ActionType,
	pub arg: String,
}

#[derive(Serialize, Deserialize)]
pub struct DefaultResponseModel<T: Serialize> {
	pub json_payload: T,
	pub action: ResponseAction,
}
