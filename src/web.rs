// pub fn read_html() -> String {
//     use std::io::Read;

//     error_chain! {
//         foreign_links {
//             Io(std::io::Error);
//             HttpRequest(reqwest::Error);
//         }
//     }

//     let mut res = reqwest::blocking::get("http://localhost:3000").expect("Can't get URL");
//     let mut body = String::new();
//     res.read_to_string(&mut body).expect("Can't parse body");

//     body
// }

//I dont need this yet
// reqwest = { version = "0.11.14", features = ["blocking"] }
