// ---------------------------- TODO 2 ------------------------------ //
// Make some enum or something and implement all the relevant stuff
//inside those enums to make the code more modular
// ------------------------------------------------------------------ //

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use mecab::Tagger;
struct RelatedInfo {
    kanjis: Vec<String>,
    readings: Vec<String>,
    glosses: Vec<String>,
}

struct VocabInfo {
    original_vocab: String,
    related_info: Vec<RelatedInfo>,
}

fn segment(text: &str) -> Vec<String> {
    let mut tagger = Tagger::new("");
    let mut vocab: Vec<String> = Vec::new();

    tagger.parse_to_node(text).iter_next().for_each(|node| {
        let stat = node.stat as i32;
        if stat.ne(&mecab::MECAB_BOS_NODE) && stat.ne(&mecab::MECAB_EOS_NODE) {
            let surface = &node.surface[..node.length as usize];
            vocab.push(surface.to_owned());
        }
    });

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

fn reading_gloss(vocab: String) {
    let true_match = jmdict::entries().find(|e| {
        e.kanji_elements().any(|k| k.text == vocab) || e.reading_elements().any(|r| r.text == vocab)
    });

    let mut entries: Vec<jmdict::Entry> = Vec::new();
    if true_match.is_none() {
        entries = jmdict::entries()
            .filter(|e| filter_matches(&vocab, e))
            .collect();
    } else {
        entries.push(true_match.unwrap());
    }

    let mut vocab_info = VocabInfo {
        original_vocab: vocab.clone(),
        related_info: Vec::new(),
    };

    entries.iter().take(3).for_each(|entry| {
        let mut kanjis = Vec::new();
        entry
            .kanji_elements()
            .for_each(|e| kanjis.push(e.text.to_string()));

        let mut readings = Vec::new();
        entry
            .reading_elements()
            .for_each(|e| readings.push(e.text.to_string()));

        let mut glosses = Vec::new();
        entry.senses().for_each(|s| {
            s.glosses()
                .filter(|e| e.language == jmdict::GlossLanguage::English)
                .for_each(|e| glosses.push(e.text.to_string()))
        });

        vocab_info.related_info.push(RelatedInfo {
            kanjis,
            readings,
            glosses,
        });
    });

    println!("\nOriginal Vocab: {}", vocab_info.original_vocab);
    for info in vocab_info.related_info {
        println!(
            "{} ({})",
            info.kanjis.first().expect("Could not retrieve kanji"),
            info.readings.first().expect("Could not retrieve reading"),
        );
        println!("Meaning: {:?}", info.glosses);
    }
}

fn is_particle(vocab: &str) -> bool {
    let black_list = [
        "ない", "こと", "ある", "する", "とも", "でき", "いい", "は", "が", "、",
    ];
    let entry = jmdict::entries().find(|e| e.reading_elements().any(|r| &r.text == &vocab));
    if black_list.contains(&vocab) {
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

pub fn kanji_info(text: &str) {
    println!("Sentence: {}", text);
    let segmented_sentence = segment(text);
    for vocab in segmented_sentence {
        if is_particle(&vocab) {
            continue;
        }
        reading_gloss(vocab);
    }

    println!("\n");
}
