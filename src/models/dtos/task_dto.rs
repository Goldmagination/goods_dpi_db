#[derive(Serialize, Deserialize)]
pub struct TaskDto {
    pub user_id: i32, // User ID provided by the client
    pub title: String,
    pub description: String,
    pub image_base64_strings: Option<Vec<String>>,
    pub category_id: Option<String>,
    pub is_flexible_timing: bool,
    pub scheduled_date: Option<String>,
    pub scheduled_time: Option<String>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
}
