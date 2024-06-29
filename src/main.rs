mod db;
mod dal {
    pub mod category_db;
    pub mod chat_db;
    pub mod professional_db;
    pub mod professional_profile_db;
    pub mod user_db;
}
mod services {
    pub mod categories_services;
    pub mod chat_services;
    pub mod firebase_service;
    pub mod professional_profile_services;
    pub mod professional_services;
    pub mod user_services;
}

// pub mod user_services{
//     pub mod user_endpoints;
//     mod user_service;
// }}
mod models {
    pub mod address;
    pub mod address_assignments;
    pub mod dtos {
        pub mod address_dto;
        pub mod chat_dto;
        pub mod message_dto;
        pub mod professional_profile_detail_dto;
        pub mod professional_profiles_dto;
        pub mod review_dto;
        pub mod subcategory_dto;
    }
    pub mod user_aggregate {
        pub mod new_user;
        pub mod user;
    }
    pub mod professional_aggregate {
        pub mod business_hour;
        pub mod new_professional;
        pub mod professional;
        pub mod professional_profile;
        pub mod service_offering;
    }
    pub mod category_aggregate {
        pub mod category;
        pub mod sub_category;
    }
    pub mod review_aggregate {
        pub mod review;
        pub mod review_content_assignments;
    }
    pub mod appointment_aggregate {
        pub mod appointment_assignment;
    }
    pub mod chat_aggregate {
        pub mod chat;
        pub mod message;
        pub mod message_assignment;
    }
}
mod schema {
    pub mod schema;
}

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use db::Pool;
use dotenv::dotenv;

use services::{
    categories_services::category_endpoints, chat_services::chat_endpoints,
    professional_profile_services::professional_profile_endpoints,
    professional_services::professional_endpoints, user_services::user_endpoints,
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
                    .max_age(3600),
            )
            .app_data(web::Data::new(pool.clone()))
            .configure(user_endpoints::user_routes)
            .configure(professional_endpoints::professional_routes)
            .configure(professional_profile_endpoints::professional_profile_routes)
            .configure(category_endpoints::category_routes)
            .configure(chat_endpoints::chat_routes)
        // TODO: add other routes here
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
