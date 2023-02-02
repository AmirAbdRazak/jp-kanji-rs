mod japanese;
use std::time::SystemTime;
fn main() {
    let single_word_time = SystemTime::now();
    let single_word = "過ぎない";
    japanese::kanji_info(single_word);
    println!(
        "Processing of single_word: {}s\n",
        single_word_time.elapsed().unwrap().as_secs_f32()
    );

    let single_sentence_time = SystemTime::now();
    let single_sentence = "結界といい人払いといい、少なくとも協力する意思があることがわかる";
    japanese::kanji_info(single_sentence);
    println!(
        "Processing of single_sentence: {}s",
        single_sentence_time.elapsed().unwrap().as_secs_f32()
    );
}
