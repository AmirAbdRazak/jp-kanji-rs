use lindera::{
    mode::Mode,
    tokenizer::{DictionaryConfig, Tokenizer, TokenizerConfig},
    DictionaryKind,
};

fn get_reading_gloss(vocab: String) {
    println!("{}", vocab);
    let entry = jmdict::entries().find(|k| k.kanji_elements().any(|k| k.text == vocab));
    // if let None = entry {
    //     entry = jmdict::entries().find(|k| k.reading_elements().any(|k| k.text == vocab));
    // }

    if let Some(entry) = entry {
        let letter_sense = entry.senses().next().expect("Failed to find sense object");

        let letter_gloss = letter_sense
            .glosses()
            .next()
            .expect("Failed to find Glossary")
            .text;

        let reading = entry
            .reading_elements()
            .next()
            .expect("No reading element")
            .text;
        println!("{vocab} ({:?}) : {}", reading, letter_gloss);
    }
}

fn get_jp_body() -> String {
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

    // println!("Status: {}", res.status());
    // println!("Headers:\n{:#?}", res.headers());
    // println!("Body:\n{}", body);

    body
}

fn parse_to_kanji(text: &str) {
    let lindera_sentence = segment_lindera(text);

    // println!("Sentence {}", text);

    println!("\nSegmentation of sentence using Lindera\n");
    for vocab in lindera_sentence {
        get_reading_gloss(vocab);
    }
}

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

    let jp_text =  "身の丈に合わない魔法だと、すぐMPが足りなくなるんだよね。要は継戦能力が足りないんだよ、マリアちゃん";
    parse_to_kanji(jp_text);

    let other_jp = "太郎は次郎が持っている本を花子に渡した。";
    parse_to_kanji(other_jp);
}
