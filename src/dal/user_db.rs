use crate::models::user_aggregate::{user::User, user::NewUser};
use crate::schema::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;


pub async fn get_user_by_email(conn: &mut PgConnection, user_email: String) -> Result<User, Error> {
    // schema::users::table.find(user_uid).first(conn)
    users.filter(email.eq(user_email)).first(conn)
}

pub fn save_user_to_database(conn: &mut PgConnection, user_name: &str, user_email: &str, user_uid_to_save: &str,) -> Result<(), Error> {
    // Create a new user instance for insertion
    let new_user = NewUser {
        name: user_name.to_string(),
        email: user_email.to_string(),
        user_uid: user_uid_to_save.to_string(),
    };

    // Insert the new user into the database
    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)?;

    Ok(())
}
