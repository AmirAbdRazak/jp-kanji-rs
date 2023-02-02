# JP to EN Web Translator

Similar to Japanese.io or Japanese10, this will give you information about a kanji that you hover. 
</br>
Developed in Rust using Mecab and JMDict libraries for everything related to Japanese processing.

<b>Project is not in Minimal Viable Product stage yet.</b>

Example from running `cargo run` on "結界といい人払いといい、少なくとも協力する意思があることがわかる": </br>
![image](https://user-images.githubusercontent.com/83165406/216344535-48f6a723-816c-49c8-a0fa-3eda1e3947a3.png)

Using `cargo build --release` on the other hand would result in `0.14492431s` processing time which I could definitely make it lower if I used JMDict's default feature which gives 30000 entries instead of using the full database, but even if it made it 100x faster the vocabulary cut isn't worth it.


---
## TODO:
## **Stage 1**
[O] Create basic sentence segmentation </br>
[O] Translate the sentence segmentation that are exact matches </br>
[O] Filter out words and more accuracy, should support incomplete vocabulary </br>
[--] Clean up code and make it modular </br>
## **Stage 2:**
[X] Send it to the web using WASM and show information by hovering </br>
[X] Create UI for it </br>
[X] Make UI dynamic </br>
## **Stage 3**
[X] Create Database for kanjis </br>
[X] Add functionality to save unknown (or any) kanji </br>
[X] Create flashcard system similar to anki to review the kanjis

## **Stage 4**
[X] Create documentation for this project </br>
