use rusqlite::{Connection, Result};
use crate::data::*;


// query example
// pub fn main() {
//     let queryword = "carry";
    
//     match get_conjugated_english(&Tense::Imperfect, queryword) {
//         Ok(res) => println!("imperfect of {} is {}", queryword, res),
//         Err(err) => eprintln!("error: {}", err),
//     }
// }


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




pub fn create_root(word: &Verb) -> String { // must borrow verb as do not want to give ownership to this function (otherwise verb will not be able to be used anywhere else)
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

pub fn apply_tense(word: &Verb, mut res: String) -> String {
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
pub fn fetch_translation(word: &Verb) -> Result<String, ()> {
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
