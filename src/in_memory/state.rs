use std::{fs::File, io::{self, BufRead}, result, sync::{Arc, Mutex, RwLock}};
use rusqlite::{params, Connection};
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
        let sqlquery = "SELECT infl_type, infl_variant FROM data where latin=?";
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

fn create_database() -> rusqlite::Result<()> {
    // Connect to or create the SQLite database file
    let conn = Connection::open("main_db.sqlite")?;

    // Create the "data" table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS data (
            latin TEXT NOT NULL,
            infl_type INTEGER NOT NULL,
            infl_variant INTEGER NOT NULL
        )",
        params![],
    )?;

    // Read the CSV file and insert data into the table
    if let Ok(file) = File::open("variantandtypelist.csv") {
        let reader = io::BufReader::new(file);

        for line in reader.lines().skip(1) {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.split(',').collect();
                let latin = parts[0].to_string();
                let infl_type = parts[1].parse::<i32>().unwrap_or(0);
                let infl_variant = parts[2].parse::<i32>().unwrap_or(0);

                conn.execute(
                    "INSERT INTO data (latin, infl_type, infl_variant) VALUES (?, ?, ?)",
                    params![latin, infl_type, infl_variant],
                )?;
            }
        }
    }

    Ok(())
}

