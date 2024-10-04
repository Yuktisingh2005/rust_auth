// src/main.rs

extern crate diesel;

pub mod schema;  
pub mod models;  

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use models::{NewUser, User, UpdateUser};

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn create_user(conn: &mut PgConnection, email: &str, password: &str) -> User {
    let new_user = NewUser {
        email: email.to_string(),
        password: password.to_string(),
        is_active: true,
    };

    diesel::insert_into(schema::users::table)
        .values(&new_user)
        .get_result(conn) 
        .expect("Error saving new user")
}

fn update_user(conn: &mut PgConnection, user_id: i32, updates: UpdateUser) -> User {
    diesel::update(schema::users::table.find(user_id))
        .set(&updates)
        .get_result(conn) 
        .expect("Error updating user")
}

fn main() {
    let mut conn = establish_connection();

  
    let user = create_user(&mut conn, "example@example.com", "password123");
    println!("Created user: {:?}", user);

   
    let updates = UpdateUser {
        email: Some("updated_email@example.com".to_string()), 
        password: None, 
        is_active: Some(false),
    };

    
    let updated_user = update_user(&mut conn, user.id, updates);
    println!("Updated user: {:?}", updated_user);
}
