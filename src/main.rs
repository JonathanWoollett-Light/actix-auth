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

// The names of the database and collection we want to use.
const DB: &str = "auth";
const COLLECTION: &str = "users";

// Run with: "cargo run <username> <password>"

#[actix_web::main]
async fn main() -> Result<()> {
    println!("Started.");

    // Gets command line arguments
    let args: Vec<String> = env::args().collect();

    // Connects to MongoDB database
    // (right now setup for connecting to atlass cluster)
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

    println!("Running...");

    // Starts server
    HttpServer::new(move || {
        App::new()
            // Adds database connection client to server data
            .data(client.clone())
            // Adds auth service
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&move_config.auth.salt.as_bytes()).secure(false), // Restrict to https?
            ))
            // Routes
            .route("/", web::get().to(status))
            .route("/user{_:/?}", web::get().to(get_user))
            .route("/user/register{_:/?}", web::post().to(register))
            .route("/user/login{_:/?}", web::get().to(get_login))
            .route("/user/login{_:/?}", web::post().to(login))
            .route("/user/logout{_:/?}", web::post().to(logout))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
