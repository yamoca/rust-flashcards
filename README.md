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

IMPLEMENT WEB BEFORE IMPLEMENTING EVERYTHING EXTRA FEATURES (IE NOW)
    - create mvp and get feedback from customers (ms douglas) asap
    - iterate fast

### notes

for axum backend docs/tutorials see axum/examples/
    - readme
    - routes-and-handlers-close-together
    - static-file-server
    - todos

check excel spreadsheets
new plan:
create sql database for words
latinword, prestranslation, perftranslation, imperftranslation, pluperftranslation, futtranslation
use this and preexisting latin rust logic to generate latin (place into db too? - but dont remove logic as new words may need to be added)
can elliminate "word" struct
keep flashcard struct - each flashcard has a front and back, and number, person, tense etc depending on word type 
there is a flaschard for each word ?
a flashcard has random attributes, and queries db to get its latin (front) and english (back) translations 

need to use rwlock and mutex to get around "the trait bound..."
look at rust book rc<t> and arc<t>

https://mk270.github.io/whitakers-words/

could theoretically create a mass database by webscraping wiktionary
https://en.wiktionary.org/w/index.php?title=Category:Latin_second_conjugation_verbs&pagefrom=DUABUS+SELLIS+SEDEO%0Aduabus+sellis+sedeo#mw-pages
reference this later https://latinwordnet.exeter.ac.uk/api



WHITTAKERS WORDS
defo way forward (see above link and github link)
/blob/master/INFLECTS.LAT contains endings for ALL verbs 
try and figure out how to reverse process (i.e at the moment, program takes word as input and ouputs attributes of that word whereas i want to use attributes of word as input and word infleciton as output)