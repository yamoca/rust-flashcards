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

// use std::net::SocketAddr;

// use axum::response::Html;
// use axum::Router;
// use axum::routing::get;

// #[tokio::main]
fn main() {
    // let routes_hello = Router::new().route(
    //     "/hello",
    //     get(|| async { Html("Hello World")})
    // );

    // let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    // println!("->> LISTENING on {:?}\n", addr);

    // axum::Server::bind(&addr)
    //     .serve(routes_hello.into_make_service())
    //     .await
    //     .unwrap();


    //let servus = Noun::new("serv".to_string(), Number::Singular, Declension::First, Gender::Masculine);
    
    verb::main();
}
