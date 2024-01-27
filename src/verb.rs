enum Person {
    First,
    Second,
    Third,
}
enum Number {
    Singular,
    Plural,
}
enum Tense {
    Pluperfect,
    Perfect,
    Imperfect,
    Present,
    Future,
}
enum Voice {
    Active,
    Passive,
}
enum Mood {
    Indicative,
    Subjunctive,
}



struct Verb {
    stem: String,
    translation: String,
    person: Person,
    number: Number,
    tense: Tense,
    voice: Voice,
    mood: Mood,
}

impl Verb {
    fn new(stem: String, translation: String, person: Person, number: Number, tense: Tense, voice: Voice, mood: Mood) -> Self {
        Verb {
            stem,
            translation,
            person,
            number,
            tense,
            voice,
            mood,
        }
    }
}

#[derive(Debug)]
struct Flaschard {
    front: String,
    back: String,
    flipped: bool,
}

impl Flaschard {
    fn new(front: String, back: String, flipped: bool) -> Self {
        Flaschard {
            front,
            back,
            flipped,
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


pub fn main() {
    println!("hello world");

    let moneo: Verb = Verb::new("mone".to_string(), "warn".to_string(), Person::First, Number::Singular, Tense::Present, Voice::Active, Mood::Indicative);
    let card1: Flaschard = Flaschard::new(fetch_latin(&moneo), fetch_translation(&moneo), false);
    println!("{:#?}", card1);
}