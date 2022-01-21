use clap::Parser;
use first_planner::workout::{TaggedWorkout, Workout};
use rusqlite::{Connection, named_params, params, Result};
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
             description text,
             distance text
         )",
        [],
    )?;

    Ok(())
}
fn insert_or_update(
    workouts: &[Workout],
    connection: &Connection,
    tag: &str,
) -> rusqlite::Result<()> {
    //INSERT OR IGNORE INTO my_table (name, age) VALUES ('Karen', 34)
    //UPDATE my_table SET age = 34 WHERE name='Karen'
   // fn insert(conn: &Connection) -> Result<()> {
        let mut stmt = connection.prepare(
            "UPDATE workouts SET description = 'overwritten' WHERE tag = ? AND week = ? AND workout_type = ?"
        )?;
        // The `rusqlite::named_params!` macro (like `params!`) is useful for heterogeneous
        // sets of parameters (where all parameters are not the same type), or for queries
        // with many (more than 32) statically known parameters.
            stmt.execute(params!["test", 16, "Interval"])?;
    // However, named parameters can also be passed like:
        //stmt.execute(&[(":key", "three"), (":val", "four")])?;
        // Or even: (note that a &T is required for the value type, currently)
        //stmt.execute(&[(":key", &100), (":val", &200)])?;
        Ok(())
   // }

   /* for w in workouts {
        let tagged = TaggedWorkout::new(w.clone(), tag.to_string());
        connection.execute(
            "INSERT OR IGNORE INTO workouts (tag, week, workout_type, pace_category, description, distance )
                      VALUES ( :tag, :week, :workout_type, :pace_category, :description, :distance)
                      UPDATE workouts SET description = :description
                      WHERE tag = :tag AND week = :week AND workout_type = :workout_type
                      ",
            to_params_named_with_fields(
                &tagged,
                &[
                    "tag",
                    "week",
                    "workout_type",
                    "pace_category",
                    "description",
                    "distance"
                ],
            )
            .unwrap()
            .to_slice()
            .as_slice(),
        )?;
    }
    Ok(())*/
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
    insert_or_update(&workouts, &conn, &args.tag)?;
    Ok(())
}
