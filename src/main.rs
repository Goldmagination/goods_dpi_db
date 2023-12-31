mod db;
mod dal{pub mod user_db; pub mod professional_db; pub mod professional_profile_db;}
mod services
{
    pub mod user_services;
    pub mod professional_services;
    pub mod firebase_service;
    pub mod professional_profile_services;}

// pub mod user_services{
//     pub mod user_endpoints;
//     mod user_service;
// }}
mod models {
    pub mod address;
    pub mod address_assignments;
    pub mod dtos{
        pub mod professional_profiles_dto;
        pub mod professional_profile_detail_dto;
        pub mod review_dto;
        pub mod address_dto;
        pub mod subcategory_dto;
    }
    pub mod user_aggregate{
        pub mod user;
        pub mod new_user;
    }
    pub mod professional_aggregate{
        pub mod professional;
        pub mod new_professional;
        pub mod professional_profile;
        pub mod service_offering;
    }
    pub mod category_aggregate{
        pub mod category;
        pub mod sub_category;
    }
    pub mod review_aggregate{
        pub mod review;
        pub mod review_content_assignments;
    }
    pub mod appointment_aggregate
    {
        pub mod appointment_assignment;
    }
}
mod schema {pub mod schema;}

use db::{Pool};
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

use services::{
    user_services::user_endpoints, 
    professional_services::professional_endpoints, 
    professional_profile_services::professional_profile_endpoints
};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool: Pool = db::establish_connection();



    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
            .allowed_origin("https://example.com")
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600)
            )
            .app_data(web::Data::new(pool.clone()))
            .configure(user_endpoints::user_routes)
            .configure(professional_endpoints::professional_routes)
            .configure(professional_profile_endpoints::professional_profile_routes)
            
            // TODO: add other routes here
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
