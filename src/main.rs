use std::io;

mod data;
use data::*;

mod utils;
use utils::*;

fn main() { 
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