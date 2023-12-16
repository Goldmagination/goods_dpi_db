use crate::models::user::User;
use crate::schema::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;


pub fn get_user_by_id(conn: &mut PgConnection, uid: String) -> Result<User, Error> {
    // schema::users::table.find(user_uid).first(conn)
    users.filter(user_uid.eq(uid)).first(conn)
}