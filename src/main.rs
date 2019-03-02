#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate tantivy;

use std::vec::Vec;

mod build_search_index;
mod search;

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
        (@arg QUERY: +required "Query")
    )
    .get_matches();

    // Get args
    let query = str::replace(matches.value_of("QUERY").unwrap(), "_", " ");

    // TODO: Conditional building of index
    build_search_index::write_persistent_index().unwrap();
    search::search(query).unwrap();
}
