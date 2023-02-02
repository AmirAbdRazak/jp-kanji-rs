// --------------------------- TODO 1-------------------------------- //
// Turn this output from this
// 過ぎ

// Original Vocab: 過ぎ
// Kanji: ["過ぎ", "過ぎる", "過ぎ去る", "過去る"]
// Readings: ["すぎ", "すぎる", "すぎさる"]
// Glosses: ["past", "too (much)", "to pass through", "to pass (of time)", "to have expired", "to exceed", "to be no more than ...", "to be excessive", "to pass"]

// To this
// Original Vocab: 過ぎ
// Kanji 1: {Kanji: ..., Reading: ..., Gloss: ...}
// Kanji 2: {Kanji: ..., Reading: ..., Gloss: ...}
// Kanji 3: {Kanji: ..., Reading: ..., Gloss: ...}

// ---------------------------- TODO 2 ------------------------------ //
// Make some enum or something and implement all the relevant stuff
//inside those enums to make the code more modular
// ------------------------------------------------------------------ //
use lindera::{
    mode::Mode,
    tokenizer::{DictionaryConfig, Tokenizer, TokenizerConfig},
    DictionaryKind,
};

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

fn segment(text: &str) -> Vec<String> {
    let mut vocab: Vec<String> = Vec::new();
    let dictionary = DictionaryConfig {
        kind: Some(DictionaryKind::UniDic),
        path: None,
    };

    let config = TokenizerConfig {
        dictionary,
        user_dictionary: None,
        mode: Mode::Normal,
        with_details: false,
    };

    // create tokenizer
    let tokenizer = Tokenizer::from_config(config).expect("Can't retrieve tokenizer configuration");

    // tokenize the text
    let tokens = tokenizer.tokenize(text).expect("Could not tokenize result");

    // output the tokens
    for token in tokens {
        vocab.push(String::from(token.text));
    }

    vocab
}

fn filter_matches(vocab: &str, e: &jmdict::Entry) -> bool {
    let matcher = SkimMatcherV2::default();
    e.kanji_elements().any(|k| {
        let true_match = k.text == vocab;
        if true_match {
            return true;
        }

        let fuzzy_match = matcher.fuzzy_match(k.text, &vocab);
        let mut val = 0;
        if fuzzy_match.is_some() {
            val = fuzzy_match.unwrap();
        };

        val > 40
    })
}

struct KanjiInfo {
    kanjis: Vec<String>,
    readings: Vec<String>,
    glosses: Vec<String>,
}

fn reading_gloss(vocab: String) {
    println!("\nOriginal Vocab: {}", vocab);

    let filtered_entries = jmdict::entries().filter(|e| filter_matches(&vocab, e));

    let mut count = 0;
    let mut kanjis: Vec<String> = Vec::new();
    let mut readings: Vec<String> = Vec::new();
    let mut glosses: Vec<String> = Vec::new();
    for entry in filtered_entries {
        if count >= 3 {
            break;
        } else {
            count += 1;

            for kanji in entry.kanji_elements() {
                kanjis.push(kanji.text.to_string());
            }

            for reading in entry.reading_elements() {
                readings.push(reading.text.to_string());
            }

            for sense in entry.senses() {
                let gloss = sense.glosses().next().expect("Failed to find glossary");
                if gloss.language == jmdict::GlossLanguage::English {
                    glosses.push(gloss.text.to_string());
                }
            }
        }
    }

    let infos = KanjiInfo {
        kanjis,
        readings,
        glosses,
    };

    println!("Kanji: {:?}", infos.kanjis);
    println!("Readings: {:?}", infos.readings);
    println!("Glosses: {:?}", infos.glosses);
}

pub fn read_html() -> String {
    use error_chain::error_chain;
    use std::io::Read;

    error_chain! {
        foreign_links {
            Io(std::io::Error);
            HttpRequest(reqwest::Error);
        }
    }

    let mut res = reqwest::blocking::get("http://localhost:3000").expect("Can't get URL");
    let mut body = String::new();
    res.read_to_string(&mut body).expect("Can't parse body");

    body
}

pub fn kanji_info(text: &str) {
    let segmented_sentence = segment(text);
    for vocab in segmented_sentence {
        println!("{}", vocab);
        if is_particle(&vocab) {
            continue;
        }
        reading_gloss(vocab);
    }
}

fn is_particle(vocab: &str) -> bool {
    let entry = jmdict::entries().find(|e| e.reading_elements().any(|r| &r.text == &vocab));
    if vocab.eq("ない") {
        return true;
    }

    if entry.is_some() {
        let mut parts_of_speech: jmdict::PartsOfSpeech =
            entry.unwrap().senses().next().unwrap().parts_of_speech();

        if parts_of_speech.any(|p| p.eq(&jmdict::PartOfSpeech::Particle)) {
            return true;
        }
    }

    false
}
