use clap::Parser;
use serde::Deserialize;
use std::error::Error;
use std::io;
use std::process;
use first_planner::workout::Workout;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    tag: String,
}

fn read_csv() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.headers() {
        let header = result;
        println!("{:?}", header);
    }
    for result in rdr.deserialize() {
        let record: Workout = result?;
        println!("{:?}", record);
    }

    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;

        println!("{:?}", record);
    }
    Ok(())
}
fn main() {
    let args = Args::parse();

    println!("Tag: {}", args.tag);
    println!("Reading csv from stdin.");

    if let Err(err) = read_csv() {
        println!("error: {}", err);
        process::exit(1);
    }
}

#[derive(Debug, Deserialize)]
struct Record {
    city: String,
    region: String,
    country: String,
    population: Option<u64>,
}

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
