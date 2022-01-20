use clap::Parser;
use first_planner::workout::Workout;
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
fn insert(workouts: &[Workout], connection: &Connection) -> rusqlite::Result<()> {
    for w in workouts {
        connection.execute(
            "INSERT INTO workouts (tag, week, workout_type, pace_category, description )
            VALUES ( 'test', :week, :workout_type, :pace_category, :description)
           ",
            to_params_named_with_fields(
                &w,
                &["week", "workout_type", "pace_category", "description"],
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

    if let Ok(result) = rdr.headers() {
        println!("{:?}", result);
    }

    for result in rdr.deserialize() {
        let record: Workout = result?;
        println!("w {:?}", record);
        results.push(record.clone());
        println!("v{:?}", &results);
    }
    Ok(results)
}

fn main() -> rusqlite::Result<()> {
    let args = Args::parse();
    let workouts: Vec<Workout>;

    println!("Tag: {}", args.tag);
    println!("Reading csv from stdin.");

    if let Ok(x) = read_csv() {
        println!("read: {:?}", x);
        workouts = x;
    } else {
        println!("error reading csv");
        process::exit(1);
    }

    //create table if it doesn't exist
    let conn = Connection::open(args.db)?;
    create_db(&conn)?;
    insert(&workouts, &conn)?;
    println!("inserted");
    Ok(())
}
