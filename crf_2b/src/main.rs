use std::sync::Arc;

use actix_web::{get, post, web, App, HttpServer, http::StatusCode};
use actix_web::{
    body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder,
};
use actix_files::NamedFile;

use libfj::robocraft2::{FactoryAPI, PortalTokenProvider, SearchResponse, SearchPayload, FactoryError};

struct SearchResultsResponder {
    results: SearchResponse,
}

// Responder
impl Responder for SearchResultsResponder {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self.results).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[post("/crf-api/search")]
async fn crf_search_post(query: web::Json<SearchPayload>, data: web::Data<Arc<FactoryAPI>>) -> impl Responder {
    match data.search((&*query).clone()).await {
        Ok(results) => {
            HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&results).unwrap())
        },
        Err(FactoryError::Protocol(e)) => {
            println!("Search error: {}", e);
            HttpResponse::InternalServerError()
                .finish()
        },
        Err(FactoryError::Response(e)) => {
            println!("Search error: {} ({})", e.error_message, e.error);
            HttpResponse::BadRequest()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&e).unwrap())
        },
        Err(FactoryError::ResponseCode(e, status)) => {
            println!("Search error: {} (status:{})", e, status);
            actix_web::HttpResponseBuilder::new(StatusCode::from_u16(status).unwrap())
                .body(e.to_string())
        }
    }
}

#[get("/crf-api/search")]
async fn crf_search_get(query: web::Query<SearchPayload>, data: web::Data<Arc<FactoryAPI>>) -> impl Responder {
    match data.search((&*query).clone()).await {
        Ok(results) => {
            HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&results).unwrap())
        },
        Err(FactoryError::Protocol(e)) => {
            println!("Search error: {}", e);
            HttpResponse::InternalServerError()
                .finish()
        },
        Err(FactoryError::Response(e)) => {
            println!("Search error: {} ({})", e.error_message, e.error);
            HttpResponse::BadRequest()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&e).unwrap())
        },
        Err(FactoryError::ResponseCode(e, status)) => {
            println!("Search error: {} (status:{})", e, status);
            actix_web::HttpResponseBuilder::new(StatusCode::from_u16(status).unwrap())
                .body(e.to_string())
        }
    }
}


#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

async fn index(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("../dist/index.html")?)
}

async fn root_level(req: HttpRequest) -> actix_web::Result<NamedFile> {
    let path: std::path::PathBuf = req.match_info().query("filename").parse()?;
    let redirect = std::path::Path::new("../dist").join(path);
    println!("redirect path: {}", redirect.display());
    Ok(NamedFile::open(redirect)?)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let factory_api = Arc::new(FactoryAPI::with_auth(
        Box::new(PortalTokenProvider::with_username("FJAPIC00L", "P4$$w0rd")
            .await.unwrap())));
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(factory_api.clone()))
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .route("/", web::get().to(index))
            .route("/{filename:.*}", web::get().to(root_level))
            //.service(actix_files::Files::new("/{filename:.*}", "../dist"))
            .service(greet)
            .service(crf_search_get)
            .service(crf_search_post)
    })
    .bind(("127.0.0.1", 45554))?
    .run()
    .await
}
