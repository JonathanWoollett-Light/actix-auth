use crate::SALT;
use argon2::Config;
use bson::oid::ObjectId;
use sailfish::TemplateOnce;
use serde::{Deserialize, Serialize};

// Data for user registration
#[derive(Serialize, Deserialize, Debug)]
pub struct UserRegister {
    pub email: String,
    pub password: String,
    pub data: String,
}

// Data for user login
#[derive(Serialize, Deserialize, Debug)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

// All user data
#[derive(Serialize, Deserialize, Clone, Debug, TemplateOnce)]
#[template(path = "user.stpl")]
pub struct User {
    pub _id: ObjectId,
    pub email: String,
    pub hash: String,
    pub data: String,
}
// Defines that we can construct `User` from `UserRegistry`
// (used when we go through the registration process)
impl From<UserRegister> for User {
    fn from(post_user: UserRegister) -> Self {
        Self {
            _id: ObjectId::new(),   // Construct new `ObjectId`
            email: post_user.email, // Sets email
            hash: argon2::hash_encoded(post_user.password.as_bytes(), SALT, &Config::default())
                .unwrap(), // hashes password
            data: post_user.data,   // Sets data
        }
    }
}
