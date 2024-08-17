use crate::models::category_aggregate::sub_category::Subcategory;
use crate::schema::schema::subcategories::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;

pub async fn get_subcategory_by_category_id(
    conn: &mut PgConnection,
    id_category: i32,
) -> Result<Vec<Subcategory>, Error> {
    subcategories
        .filter(category_id.eq(id_category))
        .load::<Subcategory>(conn)
}
