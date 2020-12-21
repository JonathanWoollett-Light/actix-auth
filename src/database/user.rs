use crate::models::*;
use crate::{COLLECTION, DB};
use bson::{doc, oid::ObjectId};
use mongodb::Client;

// Adds a user to the database
pub async fn register(
    db_client: &Client,
    user_register: UserRegister,
) -> Result<User, Box<dyn std::error::Error>> {
    // Gets database and collection
    let db = db_client.database(DB);
    let collection = db.collection(COLLECTION);

    // Constructs our full user struct (adding a unique id and hashing the password)
    let user = User::from(user_register);

    // Inserts user into database
    //
    // If for you wanted emails to be unique, you would
    //  add the unique index externally (compass/mongod) and
    //  then this insert would fail when trying to add a user
    //  with a duplicate email.
    collection
        .insert_one(
            bson::to_document(&user)?, // Converts struct to bson
            None,                      // No additional options
        )
        .await?;

    return Ok(user);
}

// Returns user data if given user info matches email and password hash
pub async fn login(
    db_client: &Client,
    email: &str,
    hash: &str,
) -> Result<Option<User>, Box<dyn std::error::Error>> {
    // Gets database and collection
    let db = db_client.database(DB);
    let collection = db.collection(COLLECTION);

    // If we find some user in our database with a matching email and hash
    if let Some(user_doc) = collection
        .find_one(
            doc! { "email": email, "hash": hash }, // Where email==email && hash==hash
            None,                                  // No addtional options
        )
        .await?
    {
        // Convert struct to bson
        let user = bson::from_document(user_doc)?;

        return Ok(user);
    }
    return Ok(None);
}

// Returns respective user data if request has neccessary cookie
pub async fn get_user(
    client: &Client,
    _id: String,
) -> Result<Option<User>, Box<dyn std::error::Error>> {
    // Gets database and collection
    let db = client.database(DB);
    let collection = db.collection(COLLECTION);

    // If we find some user with the given id
    if let Some(user_doc) = collection
        .find_one(
            doc! { "_id": ObjectId::with_string(&_id)? }, // Where _id==_id
            None,                                         // No additional options
        )
        .await?
    {
        // Convert struct to bson
        let user = bson::from_document(user_doc)?;

        return Ok(user);
    }
    return Ok(None);
}
