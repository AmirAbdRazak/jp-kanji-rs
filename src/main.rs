use lindera::{
    mode::Mode,
    tokenizer::{DictionaryConfig, Tokenizer, TokenizerConfig},
    DictionaryKind,
};

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use jmdict;

fn get_reading_gloss(vocab: String) {
    println!("\nOriginal Vocab: {}", vocab);

    let matcher = SkimMatcherV2::default();

    let _ = jmdict::entries()
        .filter(|e| {
            e.kanji_elements().any(|k| {
                let true_match = k.text == vocab;
                if true_match {
                    return true;
                }

                let fuzzy_match = matcher.fuzzy_match(k.text, &vocab);
                let mut val = 0;
                if fuzzy_match.is_some() {
                    val = fuzzy_match.unwrap();
                    if val > 40 {
                        println!("{}", val);
                    }
                };

                val > 40
            })
        })
        .inspect(|e| {
            println!("Associated Kanji");
            for kanji in e.kanji_elements() {
                println!("{}", kanji.text);
            }

            println!("Translations of Vocab");

            for letter_sense in e.senses() {
                let letter_gloss = letter_sense
                    .glosses()
                    .next()
                    .expect("Failed to find Glossary");

                if letter_gloss.language == jmdict::GlossLanguage::English {
                    println!("{}", letter_gloss.text);
                }
            }
        })
        .count();
}

fn get_jp_html() -> String {
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

fn parse_to_kanji(text: &str) {
    let lindera_sentence = segment_lindera(text);

    // println!("Sentence {}", text);

    println!("\nSegmentation of sentence using Lindera\n");
    for vocab in lindera_sentence {
        println!("{}", vocab);
        if is_particle(&vocab) {
            println!("{} is a particle", vocab);
            continue;
        }
        get_reading_gloss(vocab);
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

// fn check_all_particles() {
//     let entry = jmdict::entries();
//     for e in entry {
//         let parts = e.senses().next().unwrap().parts_of_speech();

//         for part in parts {
//             if part == jmdict::PartOfSpeech::Particle {
//                 println!("{:?}, {}", e.reading_elements().next().unwrap().text, part);
//             }
//         }
//     }
// }

fn segment_lindera(text: &str) -> Vec<String> {
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

fn main() {
    // let jp_body = get_jp_body();
    // parse_to_kanji(jp_body.as_str());

    // let jp_text =  "身の丈に合わない魔法だと、すぐMPが足りなくなるんだよね。要は継戦能力が足りないんだよ、マリアちゃん";
    // parse_to_kanji(jp_text);

    // let test = "戻らないない";
    // parse_to_kanji(test);

    // let particle1 = "ぜよ";
    // let particle2 = "にしてからが";
    // let particle3 = "ない";

    // parse_to_kanji(particle1);
    // parse_to_kanji(particle2);
    // parse_to_kanji(particle3);

    // let other_jp = "太郎は次郎が持っている本を花子に渡した。";
    // parse_to_kanji(other_jp);

    let test1 = "足りない";
    parse_to_kanji(test1);
}
