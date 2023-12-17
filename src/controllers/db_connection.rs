use crate::db::Pool;

pub async fn getDbConnection()
{
    let mut conn = db_pool.get().expect("Failed to get DB connection from pool");
}
    