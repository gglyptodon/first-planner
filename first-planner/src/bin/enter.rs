use clap::Parser;
use std::error::Error;
use std::io;
use std::process;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    tag: String,
}

fn read_csv() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());
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
