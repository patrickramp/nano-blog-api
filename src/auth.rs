use crate::structs::UserData;
use actix_web::guard::GuardContext;
use actix_web::Error;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Local};
use std::collections::HashMap;
use std::env;

// Generate a random salt for hashing passwords
fn generate_salt() -> SaltString {
    SaltString::generate(&mut OsRng)
}

// Hash the password using the Argon2 algorithm
fn hash_password(userpass: String, salt: SaltString) -> Result<String, argon2::Error> {
    // Hash the password using the Argon2 algorithm
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(userpass.as_bytes(), &salt)
        .expect("Error hashing password");
    Ok(password_hash.to_string())
}

// Extract username and password from the Authorization header
fn ext_user_pass(request: &GuardContext) -> Result<(String, String, String), Error> {
    println!("Post request received:");

    if let Some(auth) = request.head().headers().get("authorization") {
        if let Ok(auth) = auth.to_str() {
            let auth = auth.splitn(3, ":").collect::<Vec<_>>();
            if auth.is_empty() || auth.len() < 3 {
                println!("Invalid Authorization request format");
                return Err(actix_web::error::ErrorUnauthorized(
                    "Invalid Authorization header format",
                ));
            }
            Ok((
                auth[0].to_string(),
                auth[1].to_string(),
                auth[2].to_string(),
            ))
        } else {
            println!("Invalid Authorization request format");
            Err(actix_web::error::ErrorUnauthorized(
                "Invalid Authorization header format",
            ))
        }
    } else {
        Err(actix_web::error::ErrorUnauthorized(
            "Authorization header not found",
        ))
    }
}

// Lookup user in database
pub fn lookup_user(
    author: &str,
    user_map: HashMap<String, UserData>,
) -> Result<(String, String, String), Error> {
    return match user_map.get(author) {
        Some(userstruct) => {
            let password = userstruct.password.to_string();
            let exptime = userstruct.exptime.to_string();
            let user_roll = userstruct.roll.to_string();
            println!("Author {} found in database", &author);
            Ok((password, exptime, user_roll))
        }
        None => {
            println!("Author {} not found", &author);
            Err(actix_web::error::ErrorUnauthorized(
                "Incorrect username or password",
            ))
        }
    };
}

// Verify the password
fn verify_password(userpass: String, authorized_hash: String) -> Result<bool, argon2::Error> {
    // Verify the password usinig the Argon2 algorithm
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(&authorized_hash).expect("Error parsing password hash");
    Ok(argon2
        .verify_password(&userpass.as_bytes(), &parsed_hash)
        .is_ok())
}

// Guard context function to validate user
pub fn validate_user(request: &GuardContext) -> bool {
    let (author, _supplied_username, supplied_password) = ext_user_pass(&request).unwrap();

    // Define admin user from environment variables
    let default_author = env::var("AUTHOR").unwrap_or("author".to_string());
    let username = env::var("AUTHOR_USER").unwrap_or("username".to_string());
    let password = env::var("AUTHOR_PASS").unwrap_or("password".to_string());
    let exptime = (Local::now() + Duration::minutes(10))
        .format("%Y-%m-%dT%H:%M:%S")
        .to_string();
    let salt = generate_salt();
    let password = hash_password(password, salt);
    let roll = "admin".to_string();

    // Add admin data to struct
    let userdata = UserData::new(username, password.unwrap(), exptime, roll);

    // Add author to hashmap for lookup
    let mut user_db = HashMap::new();
    user_db.insert(default_author, userdata);
    //println!("Database created: {:?}", user_db);

    // Lookup author in hashmap
    let (authorized_hash, _exptime, _roll) =
        lookup_user(&author, user_db).expect("Unable to locate user!");

    // Verify supplied userpass against stored hashing
    match verify_password(supplied_password, authorized_hash) {
        Ok(true) => {
            println!("Author {} authenticated", &author);
            true
        }
        Ok(false) => {
            println!("Author {} not authenticated", &author);
            false
        }
        Err(e) => {
            println!("Error authenticating user: {}", e);
            false
        }
    }
}
