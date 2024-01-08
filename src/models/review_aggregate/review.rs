use diesel::prelude::*;
use chrono::{DateTime, Utc};
use crate::schema::schema::review;
use crate::models::professional_aggregate::professional_profile::ProfessionalProfile;

#[derive(Queryable, Identifiable, Selectable, Associations)]
#[diesel(belongs_to(ProfessionalProfile))]
#[diesel(primary_key(professional_profile_id))]
#[diesel(table_name = review)]
pub struct Review {
    pub id: i32,
    pub user_id: i32,
    pub user_name: String,
    pub professional_profile_id: i32,
    pub message: String,
    pub rate: f64,
    pub published_at: DateTime<Utc>,
}

