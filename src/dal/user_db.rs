use crate::models::user::User;
use crate::schema::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;


pub fn get_user_by_email(conn: &mut PgConnection, user_email: String) -> Result<User, Error> {
    // schema::users::table.find(user_uid).first(conn)
    users.filter(email.eq(user_email)).first(conn)
}