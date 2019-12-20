// use crate::schema::users;
// use serde::{Deserialize, Serialize};
// use validator::Validate;

// #[derive(Queryable, Serialize)]
// pub struct User {
//     pub id: i32,
//     pub username: String,
//     pub email: String,
//     pub bio: Option<String>,
//     pub image: Option<String>,
//     #[serde(skip_serializing)]
//     pub hash: String,
// }

// #[derive(Insertable)]
// #[table_name = "users"]
// pub struct NewUser<'a> {
//     pub username: &'a str,
//     pub email: &'a str,
//     pub hash: &'a str,
// }
