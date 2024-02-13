use rusqlite::{Connection, Result};
use crate::Tense;


pub fn main() {
    let queryword = "carry";
    
    match get_conjugated_english(&Tense::Imperfect, queryword) {
        Ok(res) => println!("imperfect of {} is {}", queryword, res),
        Err(err) => eprintln!("error: {}", err),
    }
}


pub fn get_conjugated_english(tense: &Tense, word: &str) -> Result<String> {
    let conn = Connection::open("output_database.sqlite")?;

    let columnname = match tense {
        Tense::Present => "present_tense",
        Tense::Perfect => "perfect_tense",
        Tense::Imperfect => "imperfect_tense",
    };

    let sqlquery = format!("SELECT {} FROM data WHERE present_tense=?1", columnname);
    let mut stmt = conn.prepare(&sqlquery)?;

    let res: String = stmt.query_row([&word], |row| {
        row.get(0)
    })?;

    Ok(res)
}