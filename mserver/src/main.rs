use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::Read;    

#[derive(Deserialize)]
struct Movie {
    title: String,
    release_year: String,
}

// Handler for POST "/add-movie" endpoint
async fn add_movie(movie: web::Json<Movie>) -> impl Responder {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("movies.txt")
        .expect("Unable to open file");

    if let Err(e) = writeln!(
        file,
        "Title: {}, ReleaseYear: {}",
        movie.title, movie.release_year
    ) {
        eprintln!("Failed to write to file: {}", e);
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().body("Movie added")
}

// Handler for GET "/" endpoint
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Server is running")
}

// get movies from file
async fn get_movies() -> impl Responder {
    let mut file = OpenOptions::new()
        .read(true)
        .open("movies.txt")
        .expect("Unable to open file");

    let mut content = String::new();
    file.read_to_string(&mut content) 
        .expect("Unable to read file");

    HttpResponse::Ok().body(content)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:8000") // Allow frontend URL
            .allowed_methods(vec!["GET", "POST"]) // Specify allowed methods
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
            .allowed_header(header::CONTENT_TYPE) // Explicitly allow content-type header
            .supports_credentials() // If your request includes credentials
            .max_age(3600); // Max cache duration for preflight requests

        App::new()
            .wrap(cors)
            .service(web::resource("/add").route(web::post().to(add_movie)))
            .route("/", web::get().to(index))
            .route("/get", web::get().to(get_movies))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
//curl -X POST -H "Content-Type: application/json" -d '{"title": "Iron man", "release_year": "2008"}' http://127.0.0.1:8080/add-movie
//curl http://127.0.0.1:8080/get