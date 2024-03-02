use std::{fs::File, io::{self, BufRead}, result, sync::{Arc, Mutex, RwLock}};
use axum::routing::delete;
use rusqlite::{ffi::SQLITE_DROP_TABLE, params, Connection};
use serde::{Deserialize, Serialize};

use crate::business_logic::data::Flashcard;

#[derive(Clone)]
pub(crate) struct AppState {
    next_id: Arc<Mutex<i32>>,
    pub flashcards: Arc<RwLock<Vec<Flashcard>>>,
}

impl AppState {
    pub fn new(flashcards: Vec<Flashcard>) -> Self {
        let max_id = flashcards.iter().map(|flashcard| flashcard.id).max().unwrap_or(0);
        Self {
            next_id: Arc::new(Mutex::new(max_id + 1)),
            flashcards: Arc::new(RwLock::new(flashcards)),
        }
    }

    pub fn get_id(&self) -> i32 {
        let mut next_id = self.next_id.lock().expect("mutex poisoned");
        let id = *next_id;
        *next_id += 1;
        id
    }
}

pub(crate) fn load_state() -> AppState {
    let flashcards = vec![Flashcard::new(1, "1+1".to_string(), "2".to_string()), Flashcard::new(1, "2+2".to_string(), "4".to_string())];

    match create_database() {
        Ok(_) => println!("db creation succesful"),
        Err(err) => eprintln!("error creating database {:?}", err),
    }
    match fetch_conjugations() {
        Ok(words) => println!("{:?}", words),
        Err(err) => eprintln!("error {:?}", err),
    }

    AppState::new(flashcards)
}

// kind of equal to word struct
#[derive(Debug)]
pub struct Data {
    latin: String,
    infl_type: i32,
    infl_variant: i32,
}


fn fetch_conjugations() -> rusqlite::Result<Vec<Data>> {
    let practice_words = vec!["habito", "traho", "audio", "moneo", "sum", "volo"];
    let conn = Connection::open("main_db.sqlite")?;
    let mut inflected_words: Vec<Data> = Vec::new();

    for word in &practice_words {
        let sqlquery = "SELECT infl_type, infl_variant FROM conjugations where latin=?";
        let mut stmt = conn.prepare(&sqlquery)?;
        let result = stmt.query_row(params![word], |row| {
            Ok(Data {
                latin: word.to_string(),
                infl_type: row.get(0)?,
                infl_variant: row.get(1)?,
            })
        });

        match result {
            Ok(data) => inflected_words.push(data),
            Err(err) => eprintln!("error querying entry: {:?}", err)
        }
    };

    Ok(inflected_words)
}

#[derive(Debug)]
struct Inflection {
    infl_id: i32,
    infl_pos: String,
    infl_person: i32,
    infl_plurality: String,
    infl_tense: String,
    infl_type: i32,
    infl_variant: i32,
    infl_stem_id: i32,
    infl_voice: String,
    infl_mood: String,
    infl_ending: String,
}


async fn insert_inflection(conn: &Connection, inflection: &Inflection) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO inflections VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            inflection.infl_id,
            inflection.infl_pos.as_str(),
            inflection.infl_person,
            inflection.infl_plurality.as_str(),
            inflection.infl_tense.as_str(),
            inflection.infl_type,
            inflection.infl_variant,
            inflection.infl_stem_id,
            inflection.infl_voice.as_str(),
            inflection.infl_mood.as_str(),
            inflection.infl_ending.as_str(),
        ],
    )?;
    Ok(())
}

async fn create_database() -> rusqlite::Result<()> { 
    // Connect to or create the SQLite database file
    let conn = Connection::open("main_db.sqlite")?;


    let table_name = "data";
    let result = conn.execute(format!("DROP TABLE IF EXISTS {};", table_name).as_str(), []);

    match result {
        Ok(_) => println!("Table dropped successfully"),
        Err(err) => eprintln!("Error dropping table: {:?}", err),
    }

    // Create the "data" table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS conjugations(
            latin TEXT NOT NULL,
            infl_type INTEGER NOT NULL,
            infl_variant INTEGER NOT NULL
        )",
        params![],
    )?;

    // UNCOMMENT THIS TO INSERT DATA LIKE 6 MILLION TIMES ACCIDENTALLY
    // Read the CSV file and insert data into the table
    // if let Ok(file) = File::open("variantandtypelist.csv") {
    //     let reader = io::BufReader::new(file);

    //     for line in reader.lines().skip(1) {
    //         if let Ok(line) = line {
    //             let parts: Vec<&str> = line.split(',').collect();
    //             let latin = parts[0].to_string();
    //             let infl_type = parts[1].parse::<i32>().unwrap_or(0);
    //             let infl_variant = parts[2].parse::<i32>().unwrap_or(0);

    //             conn.execute(
    //                 "INSERT INTO conjugations (latin, infl_type, infl_variant) VALUES (?, ?, ?)",
    //                 params![latin, infl_type, infl_variant],
    //             )?;
    //         }
    //     }
    // }
    conn.execute(
        "CREATE TABLE IF NOT EXISTS inflections (
            infl_id INTEGER PRIMARY KEY,
            infl_pos TEXT NOT NULL,
            infl_person INTEGER NOT NULL,
            infl_plurality TEXT NOT NULL,
            infl_tense TEXT NOT NULL,
            infl_type INTEGER NOT NULL,
            infl_variant INTEGER NOT NULL,
            infl_stem_id INTEGER NOT NULL,
            infl_voice TEXT NOT NULL,
            infl_mood TEXT NOT NULL,
            infl_ending TEXT NOT NULL
        )",
        params![],
    )?;

    if let Ok(file) = File::open("inflections.csv") {
        let reader = io::BufReader::new(file);

        for line in reader.lines().skip(1) {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.split(',').collect();
                let inflection = Inflection {
                    infl_id: parts[0].parse().unwrap_or(0),
                    infl_pos: parts[1].to_string(),
                    infl_person: parts[2].parse().unwrap_or(0),
                    infl_plurality: parts[3].to_string(),
                    infl_tense: parts[4].to_string(),
                    infl_type: parts[5].parse().unwrap_or(0),
                    infl_variant: parts[6].parse().unwrap_or(0),
                    infl_stem_id: parts[7].parse().unwrap_or(0),
                    infl_voice: parts[8].to_string(),
                    infl_mood: parts[9].to_string(),
                    infl_ending: parts[10].to_string(),
                };

               match insert_inflection(&conn, &inflection) {
                Ok(_) => println!("Inflection inserted successfully"),
                Err(err) => {
                    if let Some(sqlite_err) = err.downcast_ref::<rusqlite::Error>() {
                        if let rusqlite::Error::SqliteFailure(sqlite_failure, _) = sqlite_err {
                            if sqlite_failure.code == rusqlite::ErrorCode::ConstraintViolation
                                && sqlite_failure.extended_code == 1555
                            {
                                eprintln!("Error: This inflection ID already exists in the table.");
                            } else {
                                eprintln!("Error inserting inflection: {:?}", err);
                            }
                        }
                    } else {
                        eprintln!("Error inserting inflection: {:?}", err);
                    }
                }
            } 
            
            }
        }
    }

    

    Ok(())
}

