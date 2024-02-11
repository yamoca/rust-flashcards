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

impl Verb {
    fn new(stem: String, translation: String, person: Person, number: Number, conjugation: Conjugation) -> Self {
        Verb {
            stem,
            translation,
            person,
            number,
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
    let porto: Verb = Verb::new("port".to_string(), "carry".to_string(), Person::First, Number::Singular, Conjugation::First);
    let moneo: Verb = Verb::new("mon".to_string(), "warn".to_string(), Person::First, Number::Singular, Conjugation::Second);
    let traho: Verb = Verb::new("tra".to_string(), "drag".to_string(), Person::First, Number::Singular, Conjugation::Third);
    let audio: Verb = Verb::new("aud".to_string(), "hear".to_string(), Person::First, Number::Singular, Conjugation::Fourth);
    println!("{}", fetch_latin(&porto));
    println!("{}", fetch_latin(&moneo));
    println!("{}", fetch_latin(&traho));
    println!("{}", fetch_latin(&audio));
}

pub fn userloop() {
    let mut exampleword: Verb = Verb::new("mone".to_string(), "warn".to_string(), Person::First, Number::Singular, Conjugation::Second);
    let card1: Flaschard = Flaschard::new(fetch_latin(&exampleword), fetch_translation(&exampleword));
    exampleword.number = Number::Plural;
    let card2: Flaschard = Flaschard::new(fetch_latin(&exampleword), fetch_translation(&exampleword));
    exampleword.person = Person::Third;
    let card3: Flaschard = Flaschard::new(fetch_latin(&exampleword), fetch_translation(&exampleword));
        // habeo2, habito1, porto1, impero1, intellego3, invenio4,
    let list: Vec<Flaschard> = vec![card1, card2, card3];

    for item in list {
        let mut input = String::new();

        println!("{}", item.front);
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(error) => eprintln!("error reading input: {}", error)
        }

        if input.trim() == item.back {
            println!("correct");
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