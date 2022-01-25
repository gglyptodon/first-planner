#[macro_use]
extern crate diesel;

extern crate dotenv;

#[macro_use]
extern crate serde_derive;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

mod db;
//use crate::db::{create_posts, get_posts, get_unpublished_posts, publish_post};
use crate::db::{create_workouts, get_workouts};
use db::{models, schema};

fn main() {
    HttpServer::new(|| {
        App::new()
            .data(web::JsonConfig::default().limit(4096))
            .route("/", web::get().to(index))
            //.route("/unpublished/", web::get().to(unpublished))
            .route("/create", web::post().to(create))
            //.route("/publish/{id}", web::put().to(publish))
    })
        .bind("127.0.0.1:8000")
        .unwrap()
        .run()
        .unwrap();
}

fn index() -> impl Responder {
    let workouts = get_workouts();
    HttpResponse::Ok().json(workouts)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWorkout {
    week: i32,
    tag: String,
    description: String,
    distance: String,
    workout_type: String,

}

pub fn create(workout: web::Json<CreateWorkout>, req: HttpRequest) -> impl Responder {
    println!("req {:?}", req);
    println!("mod {:?}", workout);
    let result = create_workouts(&*workout.0.tag,workout.0.week, &*workout.0.description, &*workout.0.workout_type, &*workout.0.distance);
    HttpResponse::Ok().json(result);
}
/*
pub fn publish(path: web::Path<String>) -> impl Responder {
    let result = publish_post(path.to_string());
    HttpResponse::Ok().json(result)
}
*/