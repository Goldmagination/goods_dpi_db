use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SubcategoryDTO {
    pub id: i32,
    pub name: String,
    pub category_id: i32,
}
