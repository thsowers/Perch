use crate::build_search_index::{build_schema, create_index};
use serde_json::value as Json;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{DocAddress, Error, IndexReader};

pub fn search(search_term: String) -> Result<Vec<(f32, tantivy::DocAddress)>, Error> {
    // Setup fields
    let schema = build_schema();
    let index = create_index(&schema);

    let title = schema.get_field("title").unwrap();
    let author = schema.get_field("author").unwrap();
    let body = schema.get_field("body").unwrap();

    // Setup Reader
    let reader = index.reader()?;

    // Acquire a new worker
    let searcher = reader.searcher();

    // Setup query
    let query_parser = QueryParser::for_index(&index, vec![title, author, body]);
    let query = query_parser.parse_query(&search_term)?;

    // Perform our query, and return the top ten results
    let top_docs = searcher.search(&query, &TopDocs::with_limit(20))?;

    Ok(top_docs)
}

pub fn search_as_json(query: String) -> Json::Value {
    let mut result = Vec::new();
    let docs = search(query).unwrap();

    // Setup fields
    let schema = build_schema();
    let index = create_index(&schema);

    // Setup Reader
    let reader = index.reader().unwrap();

    // Acquire a new worker
    let searcher = reader.searcher();

    for (_score, doc_address) in docs {
        let retrieved_doc = searcher.doc(doc_address).unwrap();

        result.push(schema.to_named_doc(&retrieved_doc));
    }

    serde_json::to_value(result).unwrap()
}
