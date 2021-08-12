/*The commented section for the parts where i used the rustc_serialize to work with the json file.
Now using the latest Serde crate. */

/*extern crate rustc_serialize;
use rustc_serialize::json::{self,ToJson, Json};*/

extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
//use serde_json::Result;
use std::fs::File;
use std::fs::OpenOptions;

//#[derive(RustcEncodable)]
#[derive(Serialize, Deserialize)]
struct Dvd {
    name: String,
    year: u16,
    cast: String,
    length: u16,
}

/*impl ToJson for Dvd {
    fn to_json(&self) -> Json {
        Json::String(format!("{}+{}+{}+{}i", self.name, self.year, self.cast, self.length))
    }
}*/

fn json_from_str(raw: &str) -> Dvd {
    serde_json::from_str(raw).unwrap()
}

fn str_from_json(dvd: &Dvd) -> String {
    serde_json::to_string(dvd).unwrap()
}

/*fn converttojson (advd: &Dvd) -> String {

    json::encode(advd).unwrap()

}*/

fn dvds_to_file(f: &String, d: Dvd) {
    let file = OpenOptions::new().append(true).open(f).unwrap();
    serde_json::to_writer(file, &d).expect("Could not write in file.(dvds_to_file function did not work.)");
}

fn dvds_from_file(f: &String) -> Dvd {
    let file = File::open(f).unwrap();
    let deserialized_json: Dvd = serde_json::from_reader(file).unwrap();
    deserialized_json
}

/*fn main() {

    let a = Dvd {
        name: String::from("Four Weddings and a Funeral"),
        year: 1994,
        cast: String::from("Hugh Grant"),
        length: 117
    };

    let encoded = converttojson(&a);

    println!("{}", encoded);

}*/
fn main() {
    let rawdata = r#"
        {
            "name": "La La Land",
            "year": 2016,
            "cast": "Emma Stone, Ryan Gosling",
            "length": 128
        }"#;

    let mut d: Dvd = json_from_str(rawdata);

    let encoded = str_from_json(&d);

    println!("{}", encoded);

    let filename = String::from("file.json");
    dvds_to_file(&filename, d);

    d = dvds_from_file(&filename);
    println!("{}", str_from_json(&d));

}