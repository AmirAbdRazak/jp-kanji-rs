# JP to EN Web Translator

Similar to Japanese.io or Japanese10, this will give you information about a kanji that you hover. 
</br>
Developed in Rust using Lindera and JMDict libraries for everything related to Japanese processing.

Project is not in Minimal Viable Product stage yet. 

Example from running cargo run on "足りない": </br>
![image](https://user-images.githubusercontent.com/83165406/216244133-ca4185b7-2ba6-40ad-bfeb-2d761945b15d.png)


TODO:
[X] Create basic sentence segmentation </br>
[X] Translate the sentence segmentation that are exact matches </br>
[X] Filter out words and more accuracy, should support incomplete vocabulary </br>
[X] Clean up code and make it modular
[] Send it to the web using WASM and show information by hovering </br>
[] Create UI for it </br>
[] Make UI dynamic </br>
[] Create documentation for this project </br>
