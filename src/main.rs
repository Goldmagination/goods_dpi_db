mod db;
mod dal{pub mod user_db;}
mod services
{pub mod user_services; pub mod firebase_service;}

// pub mod user_services{
//     pub mod user_endpoints;
//     mod user_service;
// }}
mod models {
    pub mod user_aggregate{
        pub mod user;
        pub mod new_user;
    }
    pub mod appointment_aggregate
    {
        pub mod appointment_assignment;
    }}
mod schema {pub mod schema;}

use db::{Pool};
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

use services::user_services::user_endpoints;


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
            
            // TODO: add other routes here
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
