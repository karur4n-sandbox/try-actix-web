use actix_web::{web, App, HttpRequest, HttpServer, Responder, Result};
use serde_derive::*;

async fn hello(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[derive(Deserialize)]
struct HelloPath {
    name: String,
}

async fn hello_name(path: web::Path<HelloPath>) -> Result<String> {
    Ok(format!("Welcome {}!", path.name))
}

struct AppState {
    app_name: String,
}

async fn hello_with_state(app: web::Data<AppState>) -> Result<String> {
    Ok(format!("Hello from {}", app.app_name))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: String::from("Actix-web"),
            })
            .route("/", web::get().to(hello))
            .route("/@[{name}", web::get().to(hello_name))
            .route("/info", web::get().to(hello_with_state))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
