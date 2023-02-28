use csv::{ReaderBuilder, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string_pretty};
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    name: String,
    url: String,
    username: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Password {
    id: String,
    site: String,
    login: String,
    lowercase: bool,
    uppercase: bool,
    symbols: bool,
    numbers: bool,
    length: usize,
    counter: u64,
    version: u8,
    created: String,
    modified: String,
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: csv-to-json <input-csv> <output-json>");
        std::process::exit(1);
    }

    let input_path = PathBuf::from(&args[1]);
    let output_path = PathBuf::from(&args[2]);

    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);

    let headers = csv_reader.headers()?;
    if headers.is_empty() {
        return Err(csv::Error::from(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "CSV file is empty",
        )));
    }

    let mut passwords = vec![];
    let mut counter: u64 = 1;

    for result in csv_reader.deserialize() {
        let record: Record = result?;
        let password = Password {
            id: counter.to_string(),
            site: record.name,
            login: record.username,
            lowercase: true,
            uppercase: true,
            symbols: true,
            numbers: true,
            length: 16,
            counter: 1,
            version: 2,
            created: "2023-02-28T13:29:46Z".to_string(),
            modified: "2023-02-28T13:29:46Z".to_string(),
        };
        passwords.push(password);
        counter += 1;
    }

    let json_output = json!({
        "count": passwords.len(),
        "results": passwords,
    });

    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);
    let pretty_json = match to_string_pretty(&json_output) {
        Ok(json_string) => json_string,
        Err(err) => {
            return Err(csv::Error::from(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                err,
            )))
        }
    };

    writer.write_all(pretty_json.as_bytes())?;

    Ok(())
}
