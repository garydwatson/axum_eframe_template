use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
}