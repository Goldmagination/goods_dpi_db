use crate::schema::schema::appointment;

pub fn make_an_appointment(conn: &mut PgConnection, user_id: i32) {
    appointment::table.filter().load()
}

pub fn get_user_appointments(conn: &mut PgConnection, user_id: i32) {
    appointment::table.filter().load()
}
