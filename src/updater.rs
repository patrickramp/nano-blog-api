use std::env;
use std::fs;
use std::path::Path;

fn create_html_page(posts_directory: &str, blog_html: &str) {
    // Read the contents of the directory
    if let Ok(entries) = fs::read_dir(posts_directory) {
        // Create the HTML content
        let mut html_content = String::new();
        html_content.push_str(
        "<!DOCTYPE html>\n\
        <html lang=\"en\">\n\
        <head>\n\
            <meta charset=\"UTF-8\">\n\
            <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n\
            <meta name=\"description\" content=\"My Blog\">\n\
            <meta name=\"author\" content=\"Author\">\n\
            <meta name=\"keywords\" content=\"blog\">\n\
            <link rel=\"icon\" type=\"image/png\" href=\"https://cdn-icons-png.flaticon.com/512/3135/3135715.png\">\n\
            <title>My Blog</title>\n\
            <style>\n\
            .center{text-align: center;}\n\
            a {font-size: 2em; text-decoration: none;}\n\
            </style>\n\
        </head>\n\
        \
        <body style=\"background-color:black; color:cornsilk;\">\n\
        <header>\n\
            <div class=\"center\">\n\
                <h1>My Blog</h1>\n\
                <hr>\n\
            </div>\n\
        </header>\n\
        <main>\n\
            <h1>Blog Posts:</h1>\n");

        for entry in entries {
            if let Ok(entry) = entry {
                // Get the file name
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();
                let post_name_str = file_name_str.trim_end_matches(".html").to_string();
                let post_name_str = post_name_str.replace("-", " ");
                let ref_dir = env::var("POSTS_DIR").unwrap_or("/posts".to_string());
                // Create a link to the file
                let link = format!(
                    "<a href=\".{}/{}\">{}</a><br>\n",
                    ref_dir, file_name_str, post_name_str
                );

                // Append the link to the HTML content
                html_content.push_str(&link);
            }
        }

        // Close the HTML body and document
        html_content.push_str(
            "</main>\n\
        <footer>\n\
        </footer>\n\
        </body>\n\
        </html>",
        );

        // Write the HTML content to the output file
        if let Err(err) = fs::write(blog_html, html_content) {
            eprintln!("Error writing HTML file: {}", err);
        }
    } else {
        eprintln!("Error reading directory: {}", posts_directory);
    }
}

pub fn update_posts(posts_directory: &str, blog_html: &str) {
    // Create the posts directory if it doesn't exist
    if !Path::new(posts_directory).exists() {
        if let Err(err) = fs::create_dir(posts_directory) {
            eprintln!("Error creating posts directory: {}", err);
            return;
        }
    }

    // Call the function to create the HTML page
    create_html_page(posts_directory, blog_html);

    println!("HTML page created successfully at: {}", blog_html);
}
