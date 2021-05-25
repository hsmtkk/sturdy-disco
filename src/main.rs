use actix_web::{get, App, HttpResponse, HttpServer};
use askama::Template;

mod myerror;
mod model;

#[get("/")]
async fn index() -> Result<HttpResponse, myerror::MyError> {
    let mut entries = Vec::new();
    entries.push(model::TodoEntry{
        id: 1,
        text: "First entry".to_string(),
    });
    entries.push(model::TodoEntry{
        id: 2,
        text: "Second entry".to_string(),
    });
    let html = model::IndexTemplate{entries};
    let response_body = html.render()?;
    return Ok(HttpResponse::Ok().body(response_body));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
