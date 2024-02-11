enum Person {
    First,
    Second,
    Third,
}
enum Number {
    Singular,
    Plural,
}

enum Conjugation {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
}

struct Verb {
    stem: String,
    translation: String,
    person: Person,
    number: Number,
    conjugation: Conjugation,
}
use rand::Rng;

impl Verb {
    fn new(stem: String, translation: String, conjugation: Conjugation) -> Self {
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
            stem,
            translation,
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


fn fetch_latin(word: &Verb) -> String { // must borrow verb as do not want to give ownership to this function (otherwise verb will not be able to be used anywhere else)
    let mut res = word.stem.to_string();
    match word.conjugation {
        Conjugation::First => {
            if let (Number::Singular, Person::First) = (&word.number, &word.person) {
                // No modification needed for unchanged case
            } else {
                res.push('a');
            }
        } 
        Conjugation::Second => res.push('e'),
        Conjugation::Third => match (&word.number, &word.person) {
            (Number::Singular, Person::First) => res.push('h'),
            (Number::Plural, Person::Third) => res.push_str("hu"),
            _ => res.push_str("hi"),
        },
        Conjugation::Fourth => match (&word.number, &word.person) {
            (Number::Plural, Person::Third) => res.push_str("iu"),
            _ => res.push('i'),
        }
        Conjugation::Fifth => res.push_str("todo"),
    };
    //temporarily presume all verbs are active, indicative, present tense
    match word.number {
        Number::Singular => match word.person {
            Person::First => res.push('o'),  //weird rust string stuff (see fetch_translation() for explanation)
            Person::Second => res.push('s'),
            Person::Third => res.push('t'),
        },
        Number::Plural => match word.person {
            Person::First => res.push_str("mus"),
            Person::Second => res.push_str("tis"),
            Person::Third => res.push_str("nt"),
        },
    };
    res
}


fn fetch_translation(word: &Verb) -> String {
    match word.number {
        Number::Singular => match word.person {
            Person::First => "i ".to_string() + &word.translation, //weird rust string stuff. to use "+" first input must be String and second input must be &str
            Person::Second => "you ".to_string() + &word.translation,
            Person::Third => "he ".to_string() + &word.translation, 
        },
        Number::Plural => match word.person {
            Person::First => "we ".to_string() + &word.translation,
            Person::Second => "you (pl) ".to_string() + &word.translation,
            Person::Third => "they ".to_string() + &word.translation,
        },
    }
}

use std::io;

fn main() {
    let porto: Verb = Verb::new("port".to_string(), "carry".to_string(),  Conjugation::First);
    let moneo: Verb = Verb::new("mon".to_string(), "warn".to_string(),  Conjugation::Second);
    let traho: Verb = Verb::new("tra".to_string(), "drag".to_string(), Conjugation::Third);
    let audio: Verb = Verb::new("aud".to_string(), "hear".to_string(), Conjugation::Fourth);
    userloop(porto, traho, moneo, audio);
}

fn userloop(porto: Verb, moneo: Verb, traho: Verb, audio: Verb) {
    let card1 = Flaschard::new(fetch_latin(&porto), fetch_translation(&porto));
    let card2 = Flaschard::new(fetch_latin(&moneo), fetch_translation(&moneo));
    let card3 = Flaschard::new(fetch_latin(&traho), fetch_translation(&traho));
    let card4 = Flaschard::new(fetch_latin(&audio), fetch_translation(&audio));
    let list: Vec<Flaschard> = vec![card1, card2, card3, card4];

    for item in list {
        let mut input = String::new();

        println!("{}", item.front);
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(error) => eprintln!("error reading input: {}", error)
        }

        if input.trim() == item.back {
            println!("correct");
        } else {
            println!("incorrect");
        } 
    }

    // let mut input = String::new();
    // println!("{}", card1.front);
    // match io::stdin().read_line(&mut input) {
    //     Ok(_) => (),
    //     Err(error) => eprintln!("error reading input: {}", error)
    // }

    // if input.trim() == card1.back {
    //     println!("correct");
    // }


}