extern crate mecab;

use mecab::Tagger;

fn segment_sentence(input: &str) -> Vec<String> {
    let mut tagger = Tagger::new("");
    let mut vocab: Vec<String> = Vec::new();

    for node in tagger.parse_to_node(input).iter_next() {
        let stat = node.stat as i32;
        if stat.ne(&mecab::MECAB_BOS_NODE) && stat.ne(&mecab::MECAB_EOS_NODE) {
            let surface = &node.surface[..node.length as usize];
            // println!(
            //     "Surface {}, Bytes: {:?} : Length: {}, From {} to {}",
            //     surface,
            //     surface.as_bytes(),
            //     node.length as isize,
            //     input.len() as isize - node.surface.len() as isize,
            //     input.len() as isize - node.surface.len() as isize + node.length as isize
            // );
            if surface == "ない" && vocab.len() > 0 {
                let combined = vocab.pop().expect("There was no previous message") + "ない";

                vocab.push(combined);
            } else {
                vocab.push(surface.to_owned());
            }
        }
    }

    // println!("{:?}", vocab);
    vocab
}

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

fn main() {
    // let req: Request = Request::new("http://localhost:3000");

    // let kanji_form = "お母さん";

    // let entry = jmdict::entries()
    //     .find(|e| e.kanji_elements().any(|k| k.text == kanji_form))
    //     .unwrap();

    // let reading_form = entry.reading_elements().next().unwrap().text;
    // println!("{}", reading_form);

    let jp_text =  "身の丈に合わない魔法だと、すぐMPが足りなくなるんだよね。要は継戦能力が足りないんだよ、マリアちゃん";
    let segmented_sentence = segment_sentence(jp_text);

    println!("Sentence {}", jp_text);
    for vocab in segmented_sentence {
        get_reading_gloss(vocab);
    }

    let other_jp = "太郎は次郎が持っている本を花子に渡した。";
    let segment = segment_sentence(other_jp);

    println!("\nSentence {}", other_jp);
    for vocab in segment {
        get_reading_gloss(vocab);
    }
}
