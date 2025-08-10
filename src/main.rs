mod db;
mod websocket;
mod handlers;
mod middleware;
mod errors {
    pub mod booking_errors;
    pub mod firebase_errors;
    pub mod task_errors;
}
mod dal {
    pub mod address_db;
    pub mod booking_db;
    pub mod category_db;
    pub mod chat_db;
    pub mod professional_db;
    pub mod professional_profile_db;
    pub mod task_db;
    pub mod user_db;
}
mod services {
    pub mod categories_services;
    pub mod chat_services;
    pub mod firebase_service;
    pub mod professional_profile_services;
    pub mod professional_services;
    pub mod task_services;
    pub mod user_services;
}

mod models {
    pub mod address;
    pub mod address_assignments;
    pub mod dtos {
        pub mod address_dto;
        pub mod booking_dto;
        pub mod chat_dto;
        pub mod message_dto;
        pub mod professional_profile_detail_dto;
        pub mod professional_profiles_dto;
        pub mod review_dto;
        pub mod subcategory_dto;
        pub mod task_dto;
        pub mod user_dto;
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
    pub mod booking_aggregate {
        pub mod booking;
        pub mod booking_assignment;
        pub mod booking_status;
    }
    pub mod chat_aggregate {
        pub mod chat;
        pub mod message;
        pub mod message_assignment;
    }
    pub mod task_aggregate {
        pub mod task;
        pub mod task_assignment;
    }
}
mod schema {
    pub mod schema;
}

use actix::Actor;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer, middleware::Logger};
use db::Pool;
use dotenv::dotenv;
use env_logger::Env;
use log::info;

use services::{
    categories_services::category_endpoints, chat_services::chat_endpoints,
    professional_profile_services::professional_profile_endpoints,
    professional_services::professional_endpoints, task_services::task_endpoints,
    user_services::user_endpoints,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    // Load environment variables
    dotenv().ok();
    
    // Database connection pool
    let pool: Pool = db::establish_connection();
    
    // Start chat server actor for WebSocket
    let chat_server = websocket::ChatServer::new().start();
    
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("{}:{}", host, port);
    
    info!("Starting Goods backend with WebSocket on {}", bind_address);

    HttpServer::new(move || {
        App::new()
            // Add logger middleware
            .wrap(Logger::default())
            
            // CORS configuration
            .wrap(
                Cors::default()
                    .allowed_origin("*")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                    .allowed_headers(vec!["Content-Type", "Authorization"])
                    .supports_credentials()
                    .max_age(3600),
            )
            
            // App data
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(chat_server.clone()))
            
            // WebSocket routes
            .service(
                web::scope("/ws")
                    .route("/chat/{user_id}", web::get().to(handlers::websocket_handler::chat_websocket))
                    .route("/health", web::get().to(handlers::websocket_handler::websocket_health))
            )
            
            // Health check
            .route("/health", web::get().to(|| async { 
                actix_web::HttpResponse::Ok().json(serde_json::json!({
                    "status": "healthy",
                    "service": "goods-backend",
                    "websocket": "enabled"
                }))
            }))
            
            // Existing API routes
            .configure(user_endpoints::user_routes)
            .configure(professional_endpoints::professional_routes)
            .configure(professional_profile_endpoints::professional_profile_routes)
            .configure(category_endpoints::category_routes)
            .configure(chat_endpoints::chat_routes)
            .configure(task_endpoints::task_routes)
    })
    .bind(&bind_address)?
    .run()
    .await
}
