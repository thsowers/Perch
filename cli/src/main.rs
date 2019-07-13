#[macro_use]
extern crate clap;
use perch::{build_search_index, search};

fn main() {
    let matches = clap_app!(lyrics =>
        (version: "0.1.0")
        (author: "Tyler H. Sowers <thsowers@gmail.com>")
        (about: "Search Poems")
        (@arg index: -i "Build the poem index")
        (@arg QUERY: "Query")
    )
        .get_matches();

    if matches.is_present("index") {
        build_search_index::write_persistent_index().unwrap();
        return;
    }

    // Get args
    let query = str::replace(matches.value_of("QUERY").unwrap(), "_", " ");

    // TODO: Clean up printing
    println!("{:?}", search::search(query).unwrap());
}
