use clap::Parser;
use first_planner::workout::{TaggedWorkout, Workout};
use rusqlite::{Connection, Result};
use serde_rusqlite::*;
use std::error::Error;
use std::io;
use std::process;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    tag: String,

    #[clap(short, long, default_value = "wo.db")]
    db: String,
}

fn create_db(connection: &Connection) -> Result<(), rusqlite::Error> {
    connection.execute(
        "create table if not exists workouts (
             tag text not null,
             week integer not null,
             workout_type text not null,
             pace_category text not null,
             description text
         )",
        [],
    )?;

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
    insert(&workouts, &conn, &args.tag)?;
    Ok(())
}
