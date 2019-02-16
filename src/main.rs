#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;

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
    title: String,
    text: Vec<String>,
}


fn main() {
    let matches = clap_app!(lyrics =>
        (version: "0.1.0")
        (author: "Tyler H. Sowers <thsowers@gmail.com>")
        (about: "Search Poems")
        (@arg AUTHOR: +required "Author of poem")
        (@arg TITLE: +required "Title of poem")
    ).get_matches();

    // Get args
    let author = str::replace(matches.value_of("AUTHOR").unwrap(), "_", " ");
    let title = str::replace(matches.value_of("TITLE").unwrap(), "_", " ");

    let poems = get_poems();
    println!("{}, {}", author, title);

    // TODO: Fuzzy search and search with minimal parms
    let res = poems.iter().find(|x| x.title == title && x.author == author);

    // TODO: Pretty print
    println!("{}", serde_json::to_string_pretty(&res).unwrap());
}

fn get_poems() -> Vec<Poem> {
    let mut poem_vec: Vec<Poem> = Vec::new();

    let mut s = String::new();
    let mut data = File::open("poems.json").unwrap().read_to_string(&mut s).unwrap();
    let deserialized_poems: Poems =
        serde_json::from_str(&s).expect("error while reading json");


    for poem in deserialized_poems.poems {
        poem_vec.push(poem);
    }
    println!("Loaded {} poems ready for search", poem_vec.len());

    poem_vec
}
