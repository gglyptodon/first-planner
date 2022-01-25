/*use clap::Parser;
use first_shared::workout::{PaceCategory, TaggedWorkout, Workout, WorkoutType};
use rusqlite::{params, Connection, Result};
use serde_rusqlite::*;
use std::error::Error;
use std::io;
use std::process;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    tag: String,

    #[clap(short, long, default_value = "./app.db")]
    db: String,

    #[clap(short, long, takes_value = false)]
    update: bool,
}

fn create_db(connection: &Connection) -> Result<(), rusqlite::Error> {
    connection.execute(
        "create table if not exists workouts (
             tag text not null,
             week integer not null,
             workout_type text not null,
             pace_category text not null,
             description text,
             distance text,
            UNIQUE(tag, week, workout_type)
         )",
        [],
    )?;

    Ok(())
}
fn update(workouts: &[Workout], connection: &Connection, tag: &str) -> rusqlite::Result<()> {
    for w in workouts {
        let mut stmt = connection.prepare(
            "UPDATE workouts SET description = ?, distance =?  WHERE tag = ? AND week = ? AND workout_type = ?"
        )?;

        stmt.execute(params![
            w.description,
            w.distance,
            tag,
            w.week,
            w.workout_type.to_string()
        ])?;
    }

    Ok(())
}

fn insert(workouts: &[Workout], connection: &Connection, tag: &str) -> rusqlite::Result<()> {
    for w in workouts {
        let tagged = TaggedWorkout::new(w.clone(), tag.to_string());
        connection.execute(
            "INSERT INTO workouts (tag, week, workout_type, pace_category, description )
                      VALUES ( :tag, :week, :workout_type, :pace_category, :description)",
            to_params_named_with_fields(
                &tagged,
                &[
                    "tag",
                    "week",
                    "workout_type",
                    "pace_category",
                    "description",
                ],
            )
            .unwrap()
            .to_slice()
            .as_slice(),
        )?;
    }
    Ok(())
}

fn read_csv() -> Result<Vec<Workout>, Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut results: Vec<Workout> = Vec::new();
    let mut rdr = csv::Reader::from_reader(io::stdin());

    for result in rdr.deserialize() {
        let record: Workout = result?;
        results.push(record.clone());
    }
    Ok(results)
}

fn main() -> rusqlite::Result<()> {
    let args = Args::parse();
    let workouts: Vec<Workout>;

    println!("Info: tag: {}", args.tag);
    println!("Info: db: {}", args.db);
    println!("Info: update: {}", args.update);
    println!("Info: Reading csv from stdin."); //todo

    if let Ok(x) = read_csv() {
        workouts = x;
    } else {
        println!("error reading csv");
        process::exit(1);
    }

    //create table if it doesn't exist
    let conn = Connection::open(args.db)?;

    create_db(&conn)?;
    get_all_rows(&conn)?;
    if args.update {
        update(&workouts, &conn, &args.tag)?;
    } else if let Err(e) = insert(&workouts, &conn, &args.tag) {
        println!("Error {}. Maybe try --update", e);
        process::exit(1);
    }

    Ok(())
}

fn get_all_rows(connection: &Connection) -> rusqlite::Result<()> {
    let mut stmt = connection.prepare(
        "SELECT tag, week, description,workout_type,pace_category, distance FROM workouts",
    )?;
    let workout_iter = stmt
        .query_map([], |row| {
            let pace_cat: String = row.get(3).unwrap();
            let wo_type: String = row.get(2).unwrap();
            let tag: String = row.get(0).unwrap();

            Ok((
                tag,
                Workout {
                    week: row.get(1)?,
                    description: row.get(2)?,
                    pace_category: PaceCategory::new(pace_cat), //todo
                    workout_type: WorkoutType::new(wo_type),
                    distance: row.get(4)?,
                },
            ))
        })
        .unwrap();
    for w in workout_iter {
        println!("Found  {:?}", w.unwrap());
    }
    Ok(())
}
*/
fn main(){
    println!("todo");
}