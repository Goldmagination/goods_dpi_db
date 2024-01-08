use diesel::prelude::*;
use crate::schema::schema::review_content_assignments;
use crate::models::review_aggregate::review::Review;

#[derive(Queryable,Selectable, Identifiable, Associations)]
#[diesel(belongs_to(Review))]
#[diesel(primary_key(review_id))]
#[diesel(table_name = review_content_assignments)]
pub struct ReviewContentAssignment {
    pub id: i32,
    pub review_id: i32,
    pub image_url: String,
}