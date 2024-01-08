use diesel::prelude::*;
use crate::schema::schema::professional_profiles;

#[derive(Queryable, Identifiable, Selectable)]
#[diesel(table_name = professional_profiles)]
pub struct ProfessionalProfile {
    pub id: i32,
    pub professional_id: i32,
    pub professional_name: String,
    pub category_id: i32,
    pub credentials: Option<String>,
    pub delivery_enabled: bool,
    pub image_url: Option<String>,
    pub average_rating: Option<f64>,
    pub remote_available: bool,
}

impl ProfessionalProfile {
    pub fn update_average_rating(&mut self, ratings: &[f64]) {
        let sum_ratings: f64 = ratings.iter().sum();
        let count_ratings = ratings.len() as f64;
        self.average_rating = if count_ratings > 0.0 {
            Some(sum_ratings / count_ratings) 
        } else {
            None 
        };
    }

}
