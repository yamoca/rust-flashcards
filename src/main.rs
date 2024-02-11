enum Person {
    First,
    Second,
    Third,
}
enum Number {
    Singular,
    Plural,
}


struct Verb {
    stem: String,
    translation: String,
    person: Person,
    number: Number,
}

impl Verb {
    fn new(stem: String, translation: String, person: Person, number: Number) -> Self {
        Verb {
            stem,
            translation,
            person,
            number,
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
    //temporarily presume all verbs are active, indicative, present tense
    match word.number {
        Number::Singular => match word.person {
            Person::First => word.stem.to_string() + "o",  //weird rust string stuff (see fetch_translation() for explanation)
            Person::Second => word.stem.to_string() + "s",
            Person::Third => word.stem.to_string() + "t",
        },
        Number::Plural => match word.person {
            Person::First => word.stem.to_string() + "mus",
            Person::Second => word.stem.to_string() + "tis",
            Person::Third => word.stem.to_string() + "nt",
        },
    }
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


pub fn main() {
    let mut exampleword: Verb = Verb::new("mone".to_string(), "warn".to_string(), Person::First, Number::Singular);
    let card1: Flaschard = Flaschard::new(fetch_latin(&exampleword), fetch_translation(&exampleword));
    exampleword.number = Number::Plural;
    let card2: Flaschard = Flaschard::new(fetch_latin(&exampleword), fetch_translation(&exampleword));
    exampleword.person = Person::Third;
    let card3: Flaschard = Flaschard::new(fetch_latin(&exampleword), fetch_translation(&exampleword));

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