# JP to EN Web Translator

Similar to Japanese.io or Japanese10, this will give you information about a kanji that you hover. 
</br>
Developed in Rust using Lindera and JMDict libraries for everything related to Japanese processing.

Project is not in Minimal Viable Product stage yet. 

Example from running cargo run on "足りない": </br>
<img src="https://user-images.githubusercontent.com/83165406/216244133-ca4185b7-2ba6-40ad-bfeb-2d761945b15d.png" width=250 />


---
## TODO:
## **Stage 1**
[O] Create basic sentence segmentation </br>
[O] Translate the sentence segmentation that are exact matches </br>
[O] Filter out words and more accuracy, should support incomplete vocabulary </br>
[X] Clean up code and make it modular </br>
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
