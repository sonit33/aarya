use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ActionType {
    Redirect,
    HandleError,
    Inform,
}

#[derive(Serialize, Deserialize)]
pub struct DefaultResponseModel<T: Serialize> {
    pub action_type: ActionType,
    pub arg: T,
}
