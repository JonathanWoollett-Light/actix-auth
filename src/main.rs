mod config;
mod database;
mod handlers;
mod models;

use crate::handlers::*;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use mongodb::Client;
use std::{env, io::Result};

// Salt for encrypting passwords
const SALT: &[u8] = "=F#!AA9Ev$Ve3m@FUenH-uz?ccYkf,".as_bytes();

// "cargo run <username> <password>"

#[actix_rt::main]
async fn main() -> Result<()> {
    println!("Started.");

    // Gets command line arguments
    let args: Vec<String> = env::args().collect();

    // Connects to MongoDB database
    let uri_str = format!(
        "mongodb+srv://{}:{}@cluster0.wwsrh.mongodb.net/local?retryWrites=true&w=majority",
        args[1], args[2]
    );
    let client = Client::with_uri_str(&uri_str)
        .await
        .expect("Could not connect to database");

    println!("Connected to databse.");

    // Loads config file
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();
    let move_config = config.clone(); // This is probably a bad way of doing this

    // Starts server
    HttpServer::new(move || {
        App::new()
            .data(client.clone())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&move_config.auth.salt.as_bytes()).secure(false), // Restrict to https?
            ))
            .route("/", web::get().to(status))
            .route("/user{_:/?}", web::get().to(get_user))
            .route("/user/register{_:/?}", web::post().to(register))
            .route("/user/login{_:/?}", web::post().to(login))
            .route("/user/logout{_:/?}", web::post().to(logout))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
