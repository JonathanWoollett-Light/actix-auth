use crate::{database, models::*, SALT};
use actix_web::{web, HttpResponse, Responder};

use actix_identity::Identity;
use argon2::Config;

use mongodb::Client;
use serde::Serialize;

use sailfish::TemplateOnce;

// Basic check
pub async fn status() -> impl Responder {
    web::HttpResponse::Ok().finish()
}

// Creates new user
pub async fn register(
    db_client: web::Data<Client>,
    json: web::Json<UserRegister>,
) -> impl Responder {
    respond(
        database::register(
            db_client.get_ref(), // Gets reference to `Client`
            json.into_inner(),   // Unwraps `web::Json<UserRegister>` into `UserRegister`
        )
        .await,
    )
}

// Gets login page
pub async fn get_login() -> impl Responder {
    web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("./views/login.html"))
}

// Logs in
pub async fn login(
    db_client: web::Data<Client>,
    form: web::Form<UserLogin>,
    id: Identity,
) -> impl Responder {
    // Unwraps `web::Form<UserLogin>` into `UserLogin`
    let data = form.into_inner();

    // Hashes password
    let hash = argon2::hash_encoded(data.password.as_bytes(), SALT, &Config::default()).unwrap();

    // The error checking here is a little awkward, but it is clear.
    match database::login(db_client.get_ref(), &data.email, &hash).await {
        Ok(Some(user)) => {
            id.remember(user._id.to_string()); // Constructs cookie session
            let body = user.render_once().unwrap(); // Constructs html
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body)
        }
        Ok(None) => HttpResponse::Unauthorized()
            .content_type("text/html; charset=utf-8")
            .body(include_str!("./views/login.html")),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

// Logs out
pub async fn logout(_db_client: web::Data<Client>, id: Identity) -> impl Responder {
    id.forget(); // Destructs the cookie session
    HttpResponse::Ok().finish()
}

// Gets user data
pub async fn get_user(db_client: web::Data<Client>, id: Identity) -> impl Responder {
    // If user cookie has some id
    if let Some(_id) = id.identity() {
        return match database::get_user(db_client.get_ref(), _id).await {
            Ok(Some(user)) => {
                let body = user.render_once().unwrap();
                HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body)
            }
            Ok(None) => HttpResponse::Unauthorized().into(),
            Err(_) => HttpResponse::InternalServerError().into(),
        };
    }
    return HttpResponse::InternalServerError().into();
}

// A couple utility functions to make returns a bit cleaner
fn respond<T: Serialize, P>(result: Result<T, P>) -> HttpResponse {
    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}