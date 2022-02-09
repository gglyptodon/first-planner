pub mod models;
pub mod schema;

use actix_web::post;
use diesel::prelude::*;
use dotenv::dotenv;
use first_shared::workout;
use std::env;
use uuid::Uuid;

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
        //.filter(week.eq(1))
        .limit(5)
        .load::<Workout>(&con)
        .expect("Error loading ")
}

pub fn get_workouts_week(week_nr: i32) -> Vec<Workout> {
    let con = establish_connection();
    workouts
        .filter(week.eq(week_nr))
        .limit(5)
        .load::<Workout>(&con)
        .expect("Error loading ")
}

pub fn create_workouts(t: &str, w: i32, d: &str, wt: &str, pc: &str, dist: &str) -> String {
    let con = establish_connection();
    let uuid = Uuid::new_v4().to_hyphenated().to_string();
    let new_workout = NewWorkout {
        id: &uuid,
        tag: t,
        week: w,
        workout_type: wt,
        pace_category: pc,
        description: d,
        distance: dist,
    };
    diesel::insert_into(workouts::table)
        .values(&new_workout)
        .execute(&con)
        .expect("Error saving workout");
    println!("{}", &uuid);
    uuid
}
