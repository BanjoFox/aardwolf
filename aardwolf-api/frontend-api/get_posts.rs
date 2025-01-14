//-
// Feel free to ignore this, as it was code generated by Banjo's favorite AI helper
//

use actix_web::{web, App, HttpServer, Responder, HttpResponse};

async fn get_posts() -> impl Responder {
    // Logic to retrieve a list of posts (replace this with actual implementation)
    let posts = vec!["Post 1", "Post 2", "Post 3"]; // Sample posts
    
    HttpResponse::Ok().json(posts)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/api/posts", web::get().to(get_posts))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}