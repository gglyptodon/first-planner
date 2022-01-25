use super::schema::workouts;
use serde::Serialize;

#[derive(Serialize, Queryable)]
pub struct Workout {
    pub id: String,
    pub tag: String,
    pub week: i32,
    pub workout_type: String,
    pub description: String,
    pub distance: String,
}

#[derive(Insertable)]
#[table_name = "workouts"]
pub struct NewWorkout<'a> {
    pub id: &'a str,
    pub tag: &'a str,
    pub week: i32,
    pub workout_type: &'a str,
    pub description: &'a str,
    pub distance: &'a str,
}
