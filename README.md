# rust-flashcards

## simple flashcard cli and (hopefully web app) which declines and conjugates verbs and nouns for you
- under development
- currently hardcoded for cambridge gcse latin vocab list

## todo
- [x] add boilerplate for noun/verb structs 
- [x] create basic control flow for main functionality
- [ ] write logic for import and mass creation of new word (verb) structs
- [ ] finish implementing boring bits of verb tables etc
- [ ] handle edge cases / irregulars for irregulars look at no boilerplate live cat dead cat (if irregular, provide table struct or smth)
- [ ] create cli flashcard app functionality before implementing web
- [ ] write a front end and webserver (axum / actix-web?) to convert to web app


### notes

for axum backend docs/tutorials see axum/examples/
    - readme
    - routes-and-handlers-close-together
    - static-file-server
    - todos