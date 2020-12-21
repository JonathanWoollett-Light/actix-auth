use crate::{database, models::*, SALT};
use actix_web::{web, HttpResponse, Responder};

use actix_identity::Identity;
use argon2::Config;

use mongodb::Client;
use serde::Serialize;

// Basic check
pub async fn status() -> impl Responder {
    web::HttpResponse::Ok().finish()
}

// Creates new user
pub async fn register(
    db_client: web::Data<Client>,
    json: web::Json<UserRegister>,
) -> impl Responder {
    respond(database::register(db_client.get_ref(), json.into_inner()).await)
}

// Logs in
pub async fn login(
    db_client: web::Data<Client>,
    json: web::Json<UserLogin>,
    id: Identity,
) -> impl Responder {
    let data = json.into_inner();
    let hash_result = argon2::hash_encoded(data.password.as_bytes(), SALT, &Config::default());
    // The error checking here is awkward, but it is clear,
    //  the ways I've seen it done in other projects are not clear to me.
    // If you care to make this better without adding new directories and files,
    //  it would be appreciated.
    match hash_result {
        Ok(hash) => match database::login(db_client.get_ref(), &data.email, &hash).await {
            Ok(Some(user)) => {
                id.remember(user._id.to_string());
                HttpResponse::Ok().json(user)
            }
            Ok(None) => HttpResponse::Unauthorized().into(),
            Err(_) => HttpResponse::InternalServerError().into(),
        },
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

// Logs out
pub async fn logout(_db_client: web::Data<Client>, id: Identity) -> impl Responder {
    id.forget();
    HttpResponse::Ok().finish()
}

// Gets user data
pub async fn get_user(db_client: web::Data<Client>, id: Identity) -> impl Responder {
    // If
    if let Some(_id) = id.identity() {
        return auth_respond(database::get_user(db_client.get_ref(), _id).await);
    }
    return HttpResponse::InternalServerError().into();
}

fn respond<T: Serialize, P>(result: Result<T, P>) -> HttpResponse {
    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
fn auth_respond<T: Serialize, P>(result: Result<Option<T>, P>) -> HttpResponse {
    match result {
        Ok(Some(item)) => HttpResponse::Ok().json(item),
        Ok(None) => HttpResponse::Unauthorized().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
