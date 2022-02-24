use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn hello_world_response() -> impl Responder {
    HttpResponse::Ok().body("Hello world from controller");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello_world_response))
    })
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
