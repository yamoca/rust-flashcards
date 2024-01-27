// #[derive(Debug)]
// struct Noun {
//     stem: String,
//     number: Number,
//     declension: Declension,
//     gender: Gender,
// }

// impl Noun {
//     fn new(stem: String, number: Number, declension: Declension, gender: Gender) -> Self {
//         Noun {
//             stem,
//             number,
//             declension,
//             gender,
//         }
//     }

//     fn format(self) {
//         // todo
//         return
//     }
// }

// #[derive(Debug)]
// enum Number {
//     Singular,
//     Plural,
// }

// #[derive(Debug)]
// enum Declension {
//     First,
//     Second,
//     Third,
//     Fourth,
//     Fifth,
// }

// #[derive(Debug)]
// enum Gender {
//     Masculine,
//     Feminine,
//     Neuter,
// }

//ignore nouns for now practice with verbs instead
mod verb;

fn main() {
    //let servus = Noun::new("serv".to_string(), Number::Singular, Declension::First, Gender::Masculine);
    
    verb::main();
    // match servus.number {
    //     Number::Singular => println!("singular noun"),
    //     Number::Plural => println!("plural noun"),
    // }
}
