use crate::updater::*;
use actix_multipart::Multipart;
use actix_web::error::Error;
use actix_web::{HttpResponse, Result};
use futures_util::StreamExt;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use uuid::Uuid;

// pub async fn get_posts() -> HttpResponse {
//     // This is a protected route; only users with a valid JWT token can access it
//     HttpResponse::Ok().json("List of posts")
// }
//
// pub async fn update_post() -> HttpResponse {
//     // This is a protected route; only users with a valid JWT token can access it
//     HttpResponse::Ok().json("Update a post")
// }
//
// pub async fn delete_post() -> HttpResponse {
//     // This is a protected route; only users with a valid JWT token can access it
//     HttpResponse::Ok().json("Delete a post")
// }

pub async fn create_post(mut payload: Multipart) -> Result<HttpResponse, Error> {
    println!("File upload request detected...");

    // Load posts directory from environment variable
    let public_dir = env::var("PUBLIC_DIR").unwrap_or("./public".to_string());
    let uploads_dir = env::var("POST_DIR").unwrap_or("/posts".to_string());
    let posts_dir = format!("{}{}", public_dir, uploads_dir);
    // Create directory if it doesn't exist
    if !PathBuf::from(&posts_dir).exists() {
        if let Err(err) = std::fs::create_dir_all(posts_dir.clone()) {
            eprintln!("Error creating posts directory: {}", err);
            return Err(actix_web::error::ErrorBadRequest(
                "Failed to create directory",
            ));
        }
    }
    let blog_html = env::var("BLOG_HTML").unwrap_or("./public/blog.html".to_string());
    // Create all parent directories if they don't exist
    if let Some(parent) = PathBuf::from(&blog_html).parent() {
        if !parent.exists() {
            if let Err(err) = std::fs::create_dir_all(parent) {
                eprintln!("Error creating directories: {}", err);
                return Err(actix_web::error::ErrorBadRequest(
                    "Failed to create HTML directory directories",
                ));
            }
        }
    }
    // Create the blog.html file if it doesn't exist
    if !PathBuf::from(&blog_html).exists() {
        if let Err(err) = std::fs::File::create(blog_html.clone()) {
            eprintln!("Error creating HTML directory file: {}", err);
            return Err(actix_web::error::ErrorBadRequest(
                "Failed to create HTML directory file",
            ));
        }
    }

    // Read the multipart stream
    while let Some(item) = payload.next().await {
        let mut field = item.expect("Error processing form data");
        println!("File name: {}", field.name());

        // Get the content-disposition header to extract the filename
        let content_disposition = field.content_disposition();
        if content_disposition.is_inline() {
            return Err(actix_web::error::ErrorBadRequest(
                "File upload not allowed for inline files",
            ));
        }
        // Extract the filename from the content-disposition header
        let filename = match content_disposition.get_name() {
            Some(filename) => filename.to_string(),
            None => Uuid::new_v4().to_string(),
        };
        // Create the file path
        let file_path = PathBuf::from(format!("{}/{}", posts_dir, &filename));
        let mut file = File::create(&file_path).expect("Failed to create file");

        // Read requst data and write it to the file
        while let Some(chunk) = field.next().await {
            let data = chunk.expect("Error reading data from stream");
            file.write_all(&data).expect("Failed to write data");
            println!("File \"{}\" uploaded successfully", filename);
        }
        // Update the blog.html file
        update_posts(&posts_dir, &blog_html);

        return Ok(HttpResponse::Ok().body(format!("File \"{}\" uploaded successfully", filename)));
    }

    Err(actix_web::error::ErrorBadRequest("No file found"))
}
