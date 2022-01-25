pub mod schema;
pub mod models;




use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use actix_web::post;
use uuid::Uuid;
use first_shared::workout;

use models::{NewWorkout, Workout};
use schema::workouts;
use schema::workouts::dsl::*;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}

pub fn get_workouts() -> Vec<Workout> {
    let con = establish_connection();
    workouts
        .filter(week.eq(1))
        .limit(5)
        .load::<Workout>(&con)
        .expect("Error loading ")
}

/*pub fn get_unpublished_posts() -> Vec<Post>{
    let con = establish_connection();
    posts
        .filter(published.eq(false))
        .limit(10)
        .load::<Post>(&con)
        .expect("Error loading unpublished")
}
*/

pub fn create_workouts(t: &str, w: i32, d: &str, wt: &str, dist: &str ) -> String {
    let con = establish_connection();
    let uuid = Uuid::new_v4().to_hyphenated().to_string();
    let new_workout = NewWorkout {
        id: &uuid,
        tag: t,
        week: w,
        workout_type: wt,
        description: d,
        distance: dist
    };
    diesel::insert_into(workouts::table)
        .values(&new_workout)
        .execute(&con)
        .expect("Error saving workout");
    println!("{}",&uuid);
    uuid
}
/*
pub fn publish_post(key: String) -> usize {
    let con = establish_connection();
    diesel::update(posts.find(key))
        .set(published.eq(true))
        .execute(&con)
        .expect("Error updating post")
}*/
