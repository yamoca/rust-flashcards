
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

fn create_root(word: &Verb) -> String { // must borrow verb as do not want to give ownership to this function (otherwise verb will not be able to be used anywhere else)
    // step one: create root word e.g port goes to porta (in most cases)
    let mut principle_parts = word.principle_parts.clone();
    let mut stem = String::new();

    // come back to. diff forms have diff stems (perfect vs others - see principle parts)




    match word.tense {
        Tense::Present => {
            principle_parts[0].pop(); // removes last letter from first principle part e.g porto -> port, moneo -> mone
            stem = principle_parts[0].clone();

            match word.conjugation {
                Conjugation::First => {
                    if let (Number::Singular, Person::First) = (&word.number, &word.person) {
                        // No modification needed for unchanged case
                    } else {
                        stem.push('a');
                    }
                } 
                Conjugation::Second => stem.push('e'),
                Conjugation::Third => match (&word.number, &word.person) {
                    (Number::Singular, Person::First) => stem.push('h'),
                    (Number::Plural, Person::Third) => stem.push_str("hu"),
                    _ => stem.push_str("hi"),
                },
                Conjugation::Fourth => match (&word.number, &word.person) {
                    (Number::Plural, Person::Third) => stem.push_str("iu"),
                    _ => stem.push('i'),
                }
                Conjugation::Fifth => todo!(),
            };

        },
        Tense::Imperfect => {
            // imperfect is fine to scipt cause its regular like first principle part
            principle_parts[0].pop(); // removes last letter from first principle part e.g porto -> port, moneo -> mone
            stem = principle_parts[0].clone(); // imperfect uses the first principle part

            match word.conjugation {
                Conjugation::First => stem.push('a'),
                Conjugation::Second => stem.push('e'),
                Conjugation::Third => stem.push_str("he"),
                Conjugation::Fourth => stem.push_str("ie"),
                Conjugation::Fifth => todo!(),
            };
        },
        Tense::Perfect => {
            principle_parts[2].pop(); // perfect is 3rd principle part
            stem = principle_parts[2].clone(); // just remove last letter - portavi -> portav, traxi -> trax

            // match word.conjugation {
            //     Conjugation::First => stem.push_str("av"),
            //     Conjugation::Second => stem.push('u'),
            //     Conjugation::Third => stem.push('x'),
            //     Conjugation::Fourth => {
            //         if let (Number::Singular, Person::First) = (&word.number, &word.person) {
            //             stem.push('v')
            //         } else {
            //             stem.push_str("iv")
            //         }
            //     } 
            //     Conjugation::Fifth => todo!(),
            // }
        },
    };
    // next step
    apply_tense(word, stem)
}

fn apply_tense(word: &Verb, mut res: String) -> String {
    match word.tense {
        Tense::Present => match word.number {
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
        },
        Tense::Imperfect => match word.number {
            Number::Singular => match word.person {
                Person::First => res.push_str("bam"),  //weird rust string stuff (see fetch_translation() for explanation)
                Person::Second => res.push_str("bas"),
                Person::Third => res.push_str("bat"),
            },
            Number::Plural => match word.person {
                Person::First => res.push_str("bamus"),
                Person::Second => res.push_str("batis"),
                Person::Third => res.push_str("bant"),
            },
        },
        Tense::Perfect => match word.number {
            Number::Singular => match word.person {
                Person::First => res.push('i'),  //weird rust string stuff (see fetch_translation() for explanation)
                Person::Second => res.push_str("isti"),
                Person::Third => res.push_str("it"),
            },
            Number::Plural => match word.person {
                Person::First => res.push_str("imus"),
                Person::Second => res.push_str("istis"),
                Person::Third => res.push_str("erunt"),
            },
        },
    }
    


    res
}




//english
fn fetch_translation(word: &Verb) -> Result<String, ()> {
    let participle = match get_conjugated_english(&word.tense, word.translation.as_str()) {
        Ok(participle) => participle,
        Err(err) => {
            eprintln!("error generating english participle: {}", err);
            return Err(())
        }
    };
    let res = match word.tense {
        Tense::Present => match word.number {
            Number::Singular => match word.person {
                Person::First => format!("i {}", participle),
                Person::Second => format!("you {}", participle),
                Person::Third => format!("he {}{}", participle, "s"), //goofy ahh third person singular in english
            },
            Number::Plural => match word.person {
                Person::First => format!("we {}", participle),
                Person::Second => format!("you pl {}", participle),
                Person::Third => format!("they {}", participle),
            },
        }
        Tense::Imperfect => match word.number {
            Number::Singular => match word.person {
                Person::First => format!("i was {}", participle),
                Person::Second => format!("you were {}", participle),
                Person::Third => format!("he was {}", participle),
            },
            Number::Plural => match word.person {
                Person::First => format!("we were {}", participle),
                Person::Second => format!("you pl were {}", participle),
                Person::Third => format!("they were {}", participle),
            },
        }
        Tense::Perfect => match word.number {
            Number::Singular => match word.person {
                Person::First => format!("i {}", participle),
                Person::Second => format!("you {}", participle),
                Person::Third => format!("he {}", participle),
            },
            Number::Plural => match word.person {
                Person::First => format!("we {}", participle),
                Person::Second => format!("you pl {}", participle),
                Person::Third => format!("they {}", participle),
            },
        }

    };

    Ok(res)
    
}

use std::io;
mod db;

fn main() { 
    db::main();
    runflashcards();
}

fn runflashcards() {
    let words = vec![
        Verb::new(vec!["porto".to_string(), "portare".to_string(), "portavi".to_string(), "portatus".to_string()], "carry".to_string(), Tense::Perfect, Conjugation::First),
        Verb::new(vec!["moneo".to_string(), "monere".to_string(), "monui".to_string(), "monitus".to_string()], "warn".to_string(), Tense::Perfect, Conjugation::Second),
        Verb::new(vec!["traho".to_string(), "trahere".to_string(), "traxi".to_string(), "tractus".to_string()], "drag".to_string(), Tense::Perfect, Conjugation::Third),
        Verb::new(vec!["audio".to_string(), "audire".to_string(), "audivi".to_string(), "auditus".to_string()], "hear".to_string(), Tense::Perfect, Conjugation::Fourth),
    ];
    userloop(words);
}

fn userloop(words: Vec<Verb>) {
    let mut deck: Vec<Flaschard> = Vec::new();

    for word in words {
        match fetch_translation(&word) {
            Ok(translation) => {
                deck.push(Flaschard::new(create_root(&word), translation))
            }
            Err(()) => {
                eprintln!("failed to fetch translation for {:?}", word)
            }
        }
    }
    

    for card in deck {
        let mut input = String::new();

        println!("{}", card.front);
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(error) => eprintln!("error reading input: {}", error)
        }

        if input.trim() == card.back {
            println!("correct");
        } else {
            println!("incorrect");
            println!("correct answer: {}", card.back);
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

// today: get better way to manage and create words and flashcards and card stack (card stack is name of vector)
//    smash out tenses     
// for "he" (3rd person singular), translation is incorrect - must add -s to words e.g he warns