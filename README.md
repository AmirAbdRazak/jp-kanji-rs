# JP to EN Web Translator

Similar to Japanese.io or Japanese10, this will give you information about a kanji that you hover. 
</br>
Developed in Rust using Mecab and JMDict libraries for everything related to Japanese processing.

<b>Project is not in Minimal Viable Product stage yet.</b>

Example from running `cargo build --release` on "これだけの量の食料で一週間のキャンプに足りるでしょうか": </br>
![image](https://user-images.githubusercontent.com/83165406/216356410-3e454654-af4e-42ce-bc69-52a5df0c3af3.png)


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
