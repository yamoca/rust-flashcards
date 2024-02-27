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

## 27/2
this is kind of a devlog now for personal statement / uni whatever
changed approach back to whitakers words cause web scraping poo poo. 
talked to my dad about excel for 2 hours
changed whitakers word inflection.tsv (tab seperated values) into a pivot table and filtred to just words etc

tomorrow: change spreadsheet to include verb identification info as well as grammar flags (i forgot)
write rust program to 
get indentification (e.g conjugation etc) of a verb (using whittakers online??)
    append all the hecking inflections to the word and put them all in once big csv kinda thing (start of with og verbs porto, moneo, audio, forgot the 4th declension one)
    prolly use a db  to do this for good practice?
put into database (look into planetscale, mysql vs postreges, prisma etc)
rock and roll

# db design
follow those principles i forgot what they were (first normal form second, third)
therefore 

### all these tables need ids (for primary key)

each root / lexeme / lemma / whatever has a table (created by inflections whittakers db (aka what im working on now 27/2))

e.g
porto, portare, portavi, portatus (just using principle parts for table name atm for clearness)
person | plurality | tense | result | translation |
-------|-----------|-------|--------|-------------|
1      | singular  | pres  | porto  | i carry     |
-------|-----------|-------|--------|-------------|
2      | singular  | pres  | portes | you carry   |
-------|-----------|-------|--------|--------------

etc etc

verbs table 
e.g
verbs
root   |conjugation| link |
-------|-----------|-------
porto  | 1         | porto table 
-------|-----------|--------
maneo  | 2         | maneo table
-------|-----------|-
traho  | 3         |  etc 
-------|-----------|-
audio  | 4         | etc 


flashcard table (necessary for caching or smth??)
or necessary for keeping track of how well someone knows a particular word (word as in lexeme / root)

vocab list table (ocr gcse etc)

deck doesnt need to be a table as deck is generated on the fly (using custom options)






https://mk270.github.io/whitakers-words/

could theoretically create a mass database by webscraping wiktionary
https://en.wiktionary.org/w/index.php?title=Category:Latin_second_conjugation_verbs&pagefrom=DUABUS+SELLIS+SEDEO%0Aduabus+sellis+sedeo#mw-pages
reference this later https://latinwordnet.exeter.ac.uk/api



WHITTAKERS WORDS
defo way forward (see above link and github link)
/blob/master/INFLECTS.LAT contains endings for ALL verbs 
try and figure out how to reverse process (i.e at the moment, program takes word as input and ouputs attributes of that word whereas i want to use attributes of word as input and word infleciton as output)