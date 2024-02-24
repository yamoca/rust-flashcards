#[derive(Debug)]
enum Person {
    First,
    Second,
    Third,
}

#[derive(Debug)]
enum Number {
    Singular,
    Plural,
}

#[derive(Debug)]
enum Tense {
    Present,
    Perfect,
    Imperfect,
}

#[derive(Debug)]
enum Conjugation {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
}

#[derive(Debug)]
struct Verb {
    principle_parts: Vec<String>,
    translation: String,
    tense: Tense,
    person: Person,
    number: Number,
    conjugation: Conjugation,
}
use db::get_conjugated_english;
use rand::Rng;

impl Verb {
    fn new(principle_parts: Vec<String>, translation: String, tense: Tense, conjugation: Conjugation) -> Self {
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

#[derive(Debug)]
struct Flaschard {
    front: String,
    back: String,
}

impl Flaschard {
    fn new(front: String, back: String) -> Self {
        Flaschard {
            front,
            back,
        } 
    }
}
