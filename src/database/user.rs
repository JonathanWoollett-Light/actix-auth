use bson::{doc, oid::ObjectId};
use mongodb::Client;

use crate::models::*;

pub async fn register(
    db_client: &Client,
    user_register: UserRegister,
) -> Result<User, Box<dyn std::error::Error>> {
    let db = db_client.database("auth");
    let collection = db.collection("users");
    let user = User::from(user_register);
    //println!("{:.?}",user);
    collection
        .insert_one(bson::to_document(&user)?, None)
        .await?;
    return Ok(user);
}

pub async fn login(
    db_client: &Client,
    email: &str,
    hash: &str,
) -> Result<Option<User>, Box<dyn std::error::Error>> {
    let db = db_client.database("auth");
    let collection = db.collection("users");

    if let Some(user_doc) = collection
        .find_one(doc! { "email": email, "hash": hash }, None)
        .await?
    {
        let user = bson::from_document(user_doc)?;
        return Ok(user);
    }
    return Ok(None);
}

pub async fn get_user(
    client: &Client,
    _id: String,
) -> Result<Option<User>, Box<dyn std::error::Error>> {
    let db = client.database("auth");
    let collection = db.collection("users");

    if let Some(user_doc) = collection
        .find_one(doc! {"_id": ObjectId::with_string(&_id)? }, None)
        .await?
    {
        let user = bson::from_document(user_doc)?;
        return Ok(user);
    }
    return Ok(None);
}
