#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

extern crate dotenv;
mod db;
use crate::db::{create_workouts, get_workouts};
use crate::schema::workouts::dsl::workouts;
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use db::{models, schema};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:8080")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(index)
            .service(create)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

#[get("/workouts")]
async fn index() -> impl Responder {
    let o_workouts = get_workouts();
    HttpResponse::Ok().json(o_workouts)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWorkout {
    week: i32,
    tag: String,
    description: String,
    distance: String,
    workout_type: String,
    pace_category: String,
}
#[post("/create")]
async fn create(workout: web::Json<CreateWorkout>, req: HttpRequest) -> impl Responder {
    println!("req {:?}", req);
    println!("mod {:?}", workout);
    let result = create_workouts(
        &*workout.0.tag,
        workout.0.week,
        &*workout.0.description,
        &*workout.0.workout_type,
        &*workout.0.pace_category,
        &*workout.0.distance,
    );
    HttpResponse::Ok().json(result)
}
