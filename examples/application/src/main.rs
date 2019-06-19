use actix_web::{web, App, HttpResponse, HttpServer};

pub mod app;
pub mod combine;
pub mod state;
pub mod vh;

#[rustfmt::skip]
// <run_server>
fn main() {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/app1")
                    .route("/", web::to(|| HttpResponse::Ok())))
            .service(
                web::scope("/app2")
                    .route("/", web::to(|| HttpResponse::Ok())))
            .route("/", web::to(|| HttpResponse::Ok()))
    });
}
// </run_server>
