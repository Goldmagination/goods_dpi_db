use crate::models::professional_aggregate::{
    professional::NewProfessional, professional::Professional,
};
use crate::schema::schema::professionals::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;

pub fn get_professional_by_email(
    conn: &mut PgConnection,
    professional_email: String,
) -> Result<Professional, Error> {
    professionals
        .filter(email.eq(professional_email))
        .first(conn)
}

pub fn save_professional_to_database(
    conn: &mut PgConnection,
    professional_name: &str,
    professional_email: &str,
    professional_uid_to_save: &Uuid,
) -> Result<(), Error> {
    // Create a new professional instance for insertion
    let new_professional = NewProfessional {
        name: professional_name.to_string(),
        email: professional_email.to_string(),
        user_uid: professional_uid_to_save.to_string(),
    };

    // Insert the new user into the database
    diesel::insert_into(professionals)
        .values(&new_professional)
        .execute(conn)?;

    Ok(())
}
