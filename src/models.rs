

use diesel::prelude::*;
use serde::{Deserialize, Serialize}; /
use crate::schema::users; 
use chrono::NaiveDateTime;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub is_verified: Option<bool>,      
    pub created_at: Option<NaiveDateTime>, 
    pub is_active: bool,
}


#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub is_active: bool,
}


#[derive(AsChangeset, Debug, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub password: Option<String>,
    pub is_active: Option<bool>,
}
