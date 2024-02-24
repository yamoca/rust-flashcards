use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Person {
    First,
    Second,
    Third,
}

#[derive(Debug)]
pub enum Number {
    Singular,
    Plural,
}

#[derive(Debug)]
pub enum Tense {
    Present,
    Perfect,
    Imperfect,
}

#[derive(Debug)]
pub enum Conjugation {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
}

#[derive(Debug)]
pub struct Verb {
    pub principle_parts: Vec<String>,
    pub translation: String,
    pub tense: Tense,
    pub person: Person,
    pub number: Number,
    pub conjugation: Conjugation,
}

impl Verb {
    pub fn new(principle_parts: Vec<String>, translation: String, tense: Tense, conjugation: Conjugation) -> Self {
        let rand_person = match rand::thread_rng().gen_range(0..=2) {
            0 => Person::First,
            1 => Person::Second,
            2 => Person::Third,
            _ => unreachable!()
        };
        let rand_number = match rand::thread_rng().gen_range(0..=1) {
            0 => Number::Singular,
            1 => Number::Plural,
            _ => unreachable!()
        };
        Verb {
            principle_parts,
            translation,
            tense,
            person: rand_person,
            number: rand_number, 
            conjugation,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flashcard {
    pub id: i32,
    pub front: String,
    pub back: String,
}

impl Flashcard {
    pub fn new(id: i32, front: String, back: String) -> Self {
        Flashcard {
            id,
            front,
            back,
        } 
    }
}
