use diesel::prelude::*;


#[derive(Queryable)]
pub struct ProfessionalProfile {
    pub id: i32,
    pub professional_id: i32,
    pub category_id: i32,
    pub credentials: String,
    pub delivery_enabled: bool,
    pub image_url: Option<String>,
    pub average_rating: f32,
}

impl ProfessionalProfile {
    pub fn update_average_rating(&mut self, ratings: &[f32]) {
        let sum_ratings: f32 = ratings.iter().sum();
        let count_ratings = ratings.len() as f32;
        self.average_rating = if count_ratings > 0.0 { sum_ratings / count_ratings } else { 0.0 };
    }
}
