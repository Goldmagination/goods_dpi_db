use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> Pool {
    let database_url = env::var("DATABASE_PRODUCTION_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_establish_connection_success() {
        env::set_var("DATABASE_PRODUCTION_URL", "postgres://username:password@localhost/test_db");
        let pool = establish_connection();
        assert!(pool.get().is_ok());
    }

    #[test]
    #[should_panic(expected = "DATABASE_URL must be set")]
    fn test_establish_connection_no_env_var() {
        env::remove_var("DATABASE_PRODUCTION_URL");
        establish_connection();
    }
}
