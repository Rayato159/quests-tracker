use ntex::web;

pub async fn health_check() -> impl web::Responder {
    web::HttpResponse::Ok().body("OK")
}

pub async fn not_found() -> impl web::Responder {
    web::HttpResponse::NotFound().body("Not Found")
}
