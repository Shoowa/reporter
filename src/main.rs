use std::collections::HashMap;
use std::io::Error as ioError;

use clap::Parser;
use chrono::{DateTime};
use tokio::fs::read as readAsync;
use tokio::io::{AsyncBufReadExt, Lines};
use tokio::task;


/// A custom data type holding the command line arguments
/// ```
/// ./reporter --start="2017-05-05 03:20:00 -04:00" --end="2017-05-05 03:26:00 -04:00" ~/Downloads/log_sample.txt whatever.txt
/// ```
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Start time with timezone, inclusive
    #[clap(short, long)]
    start: String,

    /// End time with timezone, exclusive
    #[clap(short, long)]
    end: String,

    /// File names
    files: Vec<String>
}

type Table = HashMap<String, u32>;


#[tokio::main]
pub async fn main() {
    let args = Args::parse();

    let start_unix: f64 = str_to_unix(&args.start).await;
    let end_unix: f64 = str_to_unix(&args.end).await;

    // Read the files.
    let mut tasks = Vec::new();
    for f in args.files {
        let handle: task::JoinHandle<Option<Table>> = task::spawn(async move {
            if let Ok(ff) = readAsync(f).await {
                let lines = ff.lines();
                let score = parse_bytes(lines, start_unix, end_unix).await.unwrap();
                Some(score)
            } else {
                None
            }
        });

        tasks.push(handle);
    }

    // Linearly combine the results of all the HashMaps.
    let mut tally: Table  = HashMap::new();
    for handle in tasks {
        if let Ok(result) = handle.await {
            match result {
                Some(outcome) => {
                    for (k,v) in outcome.iter() {
                        tally.entry(k.to_string()).and_modify(|ultimate| {*ultimate += *v}).or_insert(*v);
                    }
                },
                None => println!("Nothing here...")
            }
        }
    }

    // Display the results.
    println!("Between {} and {}:", args.start, args.end);
    let total = tally.remove("total").unwrap() as f32;
    for (k, v) in tally.iter() {
        let p = (*v as f32 / total) * 100.0;
        println!("{} returned {:.2} 5XX errors.", k, p);
    }

}


/// Reads from a buffer line-by-line.
pub async fn parse_bytes(mut buffer: Lines<&[u8]>, start: f64, end: f64) -> Result<Table, ioError> {
    let mut amount: u32 = 0;
    let mut tally: Table = HashMap::new();

    while let Some(log) = buffer.next_line().await.unwrap() {
        let segments: Vec<&str> = log.split("|").collect();

        if let Some(metrics) = eligible(segments, start, end).await {

            // Count the total amount of eligible records.
            amount = amount + 1;

            // Count the 500s per domain.
            let (domain, status) = metrics;
            if 499 < status {
                tally.entry(domain)
                    .and_modify(|score| {*score += 1})
                    .or_insert(1);
            }
        }
    }

    // Insert total amount of eligible records into a HashMap.
    tally.insert("total".to_string(), amount);

    // Return a HashMap.
    Ok(tally)
}


/// Parse the three crucial fields and clean the data.
pub async fn eligible(log: Vec<&str>, start: f64, end: f64) -> Option< (String, u32) > {
    let timestamp: f64 = log[0].trim().parse().unwrap();

    if start <= timestamp && timestamp < end {
        let domain: String = log[2].trim().to_string();
        let status: u32 = log[4].trim().parse().unwrap();
        Some( (domain, status) )
    } else {
        None
    }
}


pub async fn str_to_unix(time: &str) -> f64 {
    DateTime::parse_from_str(&time, "%Y-%m-%d %H:%M:%S %z")
        .unwrap()
        .timestamp() as f64
}
