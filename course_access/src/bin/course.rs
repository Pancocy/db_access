use std::env;
use std::sync::Mutex;
use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;


#[path = "../states.rs"]
mod states;
#[path = "../router.rs"]
mod router;
#[path = "../handler.rs"]
mod handler;

use states::*;
use router::*;


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASEURL 未再env文件中正确声名或env文件路径错误");

    let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

    let app_data = web::Data::new(AppState{
        health_status:"success".to_string(),
        visite_count:Mutex::new(0),
        db: db_pool,
    });
    let app = move || {
        App::new()
            .app_data(app_data.clone())
            .configure(health_config)
            .configure(course_config)
    };
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
