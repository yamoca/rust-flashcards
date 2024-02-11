class Person:
    First = 1
    Second = 2
    Third = 3

class Number:
    Singular = 1
    Plural = 2

class Tense:
    Pluperfect = 1
    Perfect = 2
    Imperfect = 3
    Present = 4
    Future = 5

class Voice:
    Active = 1
    Passive = 2

class Mood:
    Indicative = 1
    Subjunctive = 2

class Verb:
    def __init__(self, stem, translation, person, number, tense, voice, mood):
        self.stem = stem
        self.translation = translation
        self.person = person
        self.number = number
        self.tense = tense
        self.voice = voice
        self.mood = mood

def fetch_latin(word):
    # Temporarily presume all verbs are active, indicative, present tense
    if word.number == Number.Singular:
        if word.person == Person.First:
            return word.stem + "o"
        elif word.person == Person.Second:
            return word.stem + "s"
        elif word.person == Person.Third:
            return word.stem + "t"
    elif word.number == Number.Plural:
        if word.person == Person.First:
            return word.stem + "mus"
        elif word.person == Person.Second:
            return word.stem + "tis"
        elif word.person == Person.Third:
            return word.stem + "nt"

def fetch_translation(word):
    # Temporarily presume all verbs are active, indicative, present tense
    if word.number == Number.Singular:
        if word.person == Person.First:
            return "i " + word.translation
        elif word.person == Person.Second:
            return "you " + word.translation
        elif word.person == Person.Third:
            return "he " + word.translation
    elif word.number == Number.Plural:
        if word.person == Person.First:
            return "we " + word.translation
        elif word.person == Person.Second:
            return "you (pl) " + word.translation
        elif word.person == Person.Third:
            return "they " + word.translation

if __name__ == "__main__":
    moneo = Verb("mone", "warn", Person.First, Number.Singular, Tense.Present, Voice.Active, Mood.Indicative)
    card1_front = fetch_latin(moneo)
    card1_back = fetch_translation(moneo)

    array = [[card1_front, card1_back]]
    array.append([fetch_latin(Verb("mone", "warn", Person.Second, Number.Singular, Tense.Present, Voice.Active, Mood.Indicative)), fetch_translation(Verb("mone", "warn", Person.Second, Number.Singular, Tense.Present, Voice.Active, Mood.Indicative))])

    for card_set in array:
        user_input = input(card_set[0]) 

        if user_input.strip() == card_set[1]:
            print("correct")
    