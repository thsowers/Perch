#[macro_use]
extern crate clap;
use clap::AppSettings;
use perch::search::SearchType;
use perch::{build_search_index, search};

fn main() {
    let matches = clap_app!(lyrics =>
        (version: "0.1.0")
        (author: "Tyler H. Sowers <thsowers@gmail.com>")
        (about: "Search Poems")
        (setting: AppSettings::ArgRequiredElseHelp)
        (@arg index: -i "Build the poem index")
        (@arg QUERY: "Query")
    )
    .get_matches();

    if matches.is_present("index") {
        println!("{}", "Building search index...");
        build_search_index::write_persistent_index().unwrap();
        println!("{}", "Success!");
        return;
    }

    // Get args
    let query = str::replace(matches.value_of("QUERY").unwrap(), "_", " ");

    // TODO: Clean up printing
    println!("{:?}", search::search(query, SearchType::JSON));
}
