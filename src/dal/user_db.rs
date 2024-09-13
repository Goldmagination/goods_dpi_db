use crate::models::dtos::user_dto::UserDTO;
use crate::models::user_aggregate::{user::NewUser, user::User};
use crate::schema::schema::bookings::{self, customer_uid};
use crate::schema::schema::message::dsl::*;
use crate::schema::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;

pub async fn get_user_by_email(
    conn: &mut PgConnection,
    user_email: &str,
) -> Result<UserDTO, diesel::result::Error> {
    let user: User = users.filter(email.eq(user_email)).first(conn)?;
    let unread_messages = message
        .filter(is_read.eq(false))
        .filter(receiver_uid.eq(&user.user_uid))
        .count()
        .get_result::<i64>(conn)?;
    let active_bookings = bookings::table
        .filter(bookings::status.eq_any(vec![1, 4, 7]))
        .filter(bookings::customer_uid.eq(&user.user_uid))
        .count()
        .get_result::<i64>(conn)?;

    let user_dto: UserDTO = UserDTO::new(user, unread_messages, active_bookings);
    Ok(user_dto)
}

pub fn save_user_to_database(
    conn: &mut PgConnection,
    user_name: &str,
    user_email: &str,
    user_uid_to_save: &str,
) -> Result<(), Error> {
    // Create a new user instance for insertion
    let new_user = NewUser {
        name: user_name.to_string(),
        email: user_email.to_string(),
        user_uid: user_uid_to_save.to_string(),
    };

    // Insert the new user into the database
    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use diesel::pg::PgConnection;
//     use crate::db::establish_connection;

//     #[test]
//     fn test_get_user_by_email() {
//         let connection = setup_test_db();

//         // Your known_category_id should be defined here
//         let test_user_data = "test@test.com";

//         let result = match get_user_by_email(&mut connection, test_user_data.to_string()){
//             Ok(user) => Ok(()),
//             Err(_) => panic!()};

//         assert!(result.is_ok());
//     }

//     // Function to setup a test database connection
//     fn setup_test_db() -> PgConnection {
//         let connection = establish_connection(); // Use your connection setup function here
//         connection.test_transaction().unwrap()
//     }
// }
