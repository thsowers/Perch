#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::fs::File;
use serde_json::Value;
use std::vec::Vec;
use std::fs;
use std::collections::HashMap;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
struct Poems {
    poems: Vec<Poem>
}

#[derive(Serialize, Deserialize, Debug)]
struct Poem {
    author: String,
    reference: String,
    title: String,
    text: Vec<String>,
}

fn main() {
    let mut poem_vec: Vec<&Poem> = Vec::new();

    let mut s = String::new();
    let mut data = File::open("poems.json").unwrap().read_to_string(&mut s).unwrap();
    let deserialized_poems: Poems =
        serde_json::from_str(&s).expect("error while reading json");


    for (i, poem) in deserialized_poems.poems.iter().enumerate() {
        poem_vec.insert(i, poem);
    }
    println!("Finished adding poem_vec");
    println!("{}", poem_vec.len());
}
