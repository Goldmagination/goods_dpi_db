use chrono::NaiveTime;
use diesel::prelude::*;
use crate::schema::schema::business_hours;

#[derive(Queryable, Identifiable, Selectable)]
#[diesel(belongs_to(ProfessionalProfile))]
#[diesel(primary_key(professional_profile_id))]
#[diesel(table_name = business_hours)]
pub struct BusinessHours {
    pub id: i32,
    pub professional_profile_id: i32,
    pub day_of_week: i32,
    pub opening_time: Option<NaiveTime>,
    pub closing_time: Option<NaiveTime>,
    pub is_available: bool,
}
