#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;

use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::vec::Vec;

use fst::{IntoStreamer, Set, SetBuilder, Streamer};
use fst_levenshtein::Levenshtein;

#[derive(Serialize, Deserialize, Debug)]
struct Poems {
    poems: Vec<Poem>,
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
    )
    .get_matches();

    // Get args
    let author = str::replace(matches.value_of("AUTHOR").unwrap(), "_", " ");
    let title = str::replace(matches.value_of("TITLE").unwrap(), "_", " ");

    generate_finite_state_machine();

    read_finite_state_machine(author, title);

    // TODO: Pretty print
    //println!("{:?}", &kvs.to_vec());
}

fn read_finite_state_machine(author: String, title: String) {
    println!("Reading stuff");
    let author_lev = Levenshtein::new(&author, 2).unwrap();
    let title_lev = Levenshtein::new(&title, 2).unwrap();

    let author_set = unsafe { Set::from_path("authors.fst").unwrap() };
    let mut author_stream = author_set.search(author_lev).into_stream();

    let title_set = unsafe { Set::from_path("titles.fst").unwrap() };
    let mut title_stream = title_set.search(title_lev).into_stream();

    let authors = author_stream.into_strs().unwrap();
    let titles = title_stream.into_strs().unwrap();

    println!(
        "{} Authors found, {} Poems found",
        authors.len(),
        titles.len()
    );
    println!("{:?}", authors);
    println!("{:?}", titles);
}

fn generate_finite_state_machine() -> io::Result<()> {
    let mut s = String::new();
    let mut data = File::open("poems.json")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    let deserialized_poems: Poems = serde_json::from_str(&s).expect("error while reading json");

    let mut authors = Vec::new();
    let mut titles = Vec::new();
    //let mut poems = Vec::new();

    for poem in deserialized_poems.poems {
        authors.push(poem.author);
        titles.push(poem.title);
        //poems.push(&poem.author.split(&poem.title));
    }

    // All items must be inserted in lexicographical order
    authors.sort();
    authors.dedup();
    titles.sort();
    titles.dedup();
    //poems.sort();

    // Setup writers
    let mut author_wrt = io::BufWriter::new(File::create("authors.fst").unwrap());
    let mut title_wrt = io::BufWriter::new(File::create("titles.fst").unwrap());
    //let mut poems_wrt = io::BufWriter::new(File::create("titles.fst").unwrap());

    let mut author_build = SetBuilder::new(author_wrt).unwrap();
    let mut title_build = SetBuilder::new(title_wrt).unwrap();
    //let mut poem_build = SetBuilder::new(poems_wrt).unwrap();

    for author in authors {
        author_build.insert(author);
    }

    for title in titles {
        title_build.insert(title);
    }

    println!("Loaded {} poems ready for search", "12");
    // If you want the writer back, then call `into_inner`. Otherwise, this
    // will finish construction and call `flush`.
    title_build.finish().unwrap();
    author_build.finish().unwrap();
    Ok(())
}
