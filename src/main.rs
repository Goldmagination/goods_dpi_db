use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

mod db;
mod dal{pub mod user_db;}
mod controllers {pub mod user_controller;}
mod models {pub mod user;}
mod schema {pub mod schema;}

pub use db::{Pool, establish_connection};

use controllers::user_controller;


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
            .route("/auth/users/{uid}", 
            web::get().to(user_controller::get_user_handler))
            // Add other routes here
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
