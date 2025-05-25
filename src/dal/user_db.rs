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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::establish_connection_test;
    use crate::models::user_aggregate::new_user::NewUser; // Corrected path
    use crate::schema::schema::users;
    use diesel::Connection;
    use diesel::RunQueryDsl;

    // Function to setup a test database connection
    fn setup_test_db() -> PgConnection {
        let mut connection = establish_connection_test();
        connection
            .begin_test_transaction()
            .expect("Failed to begin test transaction");
        connection
    }

    #[test]
    fn test_get_user_by_email() {
        let mut conn = setup_test_db();

        // Test 1: User Exists
        let test_email = "test@example.com";
        let test_name = "Test User";
        let test_uid = "testuid123";

        let new_user = NewUser {
            name: test_name.to_string(),
            email: test_email.to_string(),
            user_uid: test_uid.to_string(),
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(&mut conn)
            .expect("Failed to insert test user");

        match get_user_by_email(&mut conn, test_email) { // Removed .await
            Ok(user_dto) => {
                assert_eq!(user_dto.user.email, test_email);
                assert_eq!(user_dto.user.name, test_name);
                assert_eq!(user_dto.user.user_uid, test_uid);
                // We expect 0 unread messages and 0 active bookings for a new user
                assert_eq!(user_dto.unread_messages, 0);
                assert_eq!(user_dto.active_bookings, 0);
            }
            Err(e) => panic!("Expected user to be found, but got error: {}", e),
        }

        // Test 2: User Does Not Exist
        let non_existent_email = "nonexistent@example.com";
        match get_user_by_email(&mut conn, non_existent_email) { // Removed .await
            Ok(_) => panic!("Expected user not to be found, but got a user"),
            Err(diesel::result::Error::NotFound) => {
                // This is the expected outcome
            }
            Err(e) => panic!(
                "Expected NotFound error, but got a different error: {}",
                e
            ),
        }
    }
}
