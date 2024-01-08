use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::models::
{
    review_aggregate::review::*,
    review_aggregate::review_content_assignments::*
};

#[derive(Debug, Serialize, Deserialize, )]
pub struct ReviewDTO {
    pub id: i32,
    pub user_id: i32,
    pub user_name: String,
    pub professional_profile_id: i32,
    pub message: String,
    pub rate: f64,
    pub content_assignments: Option<Vec<ReviewContentAssignmentDTO>>
}


#[derive(Queryable, Debug, Serialize, Deserialize,)]
pub struct ReviewContentAssignmentDTO {
    pub review_id: i32,
    pub image_url: String,
}
fn from_review_assignments(review_assignments: &Vec<ReviewContentAssignment>) -> Vec<ReviewContentAssignmentDTO> {
    review_assignments.iter().map(|ra| ReviewContentAssignmentDTO {
        review_id: ra.review_id,
        image_url: ra.image_url.clone(),
    }).collect()
}

impl ReviewDTO {
    pub fn review_to_dto(review: &Review, review_assignments: &Vec<ReviewContentAssignment>) -> ReviewDTO {
        let content_assignments = if review_assignments.is_empty() {
            None
        } else {
            Some(from_review_assignments(review_assignments))
        };

        ReviewDTO {
            id: review.id,
            user_id: review.user_id,
            user_name: review.user_name.clone(),
            professional_profile_id: review.professional_profile_id,
            message: review.message.clone(),
            rate: review.rate,
            content_assignments,
        }
    }
}
