use crate::SALT;
use argon2::Config;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRegister {
    pub email: String,
    pub password: String,
    pub data: String,
}
#[derive(Serialize, Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub _id: ObjectId,
    pub email: String,
    pub hash: String,
    pub data: String,
}
impl From<UserRegister> for User {
    fn from(post_user: UserRegister) -> Self {
        Self {
            _id: ObjectId::new(),
            email: post_user.email,
            hash: argon2::hash_encoded(post_user.password.as_bytes(), SALT, &Config::default())
                .unwrap(), // TODO Do something other than `.unwrap` here
            data: post_user.data,
        }
    }
}
