mod handlers;

use actix_web::{HttpServer, App};
use log::LevelFilter;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    log::info!("server initialized and running at port 8088");
    HttpServer::new(move || {        
        App::new()
            //.service(service)
        })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
