mod auth;
mod handlers;
mod structs;
mod updater;

use crate::auth::*;
use crate::handlers::*;
use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::{guard, guard::fn_guard, web, App, HttpServer};
use num_cpus;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Define server parameters from environment variables
    let bind_to = env::var("BIND_TO").unwrap_or("0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or("8080".to_string());
    let http_dir = env::var("PUBLIC_DIR").unwrap_or("./public".to_string());
    let index = env::var("INDEX").unwrap_or("blog.html".to_string());
    let domain = env::var("WEB_DOMAIN").unwrap_or("localhost".to_string());
    let mount = env::var("MOUNT").unwrap_or("/".to_string());
    // Display server configuration to console
    println!(
        "Starting Actix Webserver...\n Listening on: {}:{}",
        bind_to, port
    );
    println!(" Serving files from directory \"{}\"", http_dir);
    println!(
        " Primary domain is \"{}\" serving \"{}\" at \"{}\"",
        domain, index, mount
    );
    // Create an Actix web server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(web::resource("/post").post(create_post))
                    .guard(fn_guard(validate_user)),
            )
            .service(
                Files::new(&mount, &http_dir)
                    // Apply guard to restrict access to specified hostname (prevent hotlinks)
                    .guard(guard::Host(&domain))
                    // Index file name
                    .index_file(&index),
            )
    })
    .workers(num_cpus::get())
    .bind(format!("{}:{}", bind_to, port))
    .expect("Failed to bind address")
    .run()
    .await
}
