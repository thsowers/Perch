use crate::build_search_index::{build_schema, create_index};
use crate::deserializer::PoemMap;
use regex::Regex;
use serde_json::value as Json;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{DocAddress, Index, IndexReader};

pub enum SearchType {
    HTML,
    JSON,
}

#[derive(Serialize, Deserialize)]
pub enum ReturnTypes {
    HTML(Json::Value),
    JSON(Json::Value),
}

fn setup_parsing() -> (Schema, IndexReader, Index) {
    // Setup fields
    let schema = build_schema();
    let index = create_index(&schema);

    // Setup Reader
    let reader = index.reader().unwrap();

    // Acquire a new worker
    let searcher = reader;

    (schema, searcher, index)
}

pub fn search(search_term: String, search_type: SearchType) -> ReturnTypes {
    let (schema, searcher, index) = setup_parsing();

    // Setup fields
    let title = schema.get_field("title").unwrap();
    let author = schema.get_field("author").unwrap();
    let body = schema.get_field("body").unwrap();

    // Setup query
    let query_parser = QueryParser::for_index(&index, vec![title, author, body]);
    let query = query_parser.parse_query(&search_term).unwrap();

    // Perform our query, and return the top ten results
    let top_docs = searcher
        .searcher()
        .search(&query, &TopDocs::with_limit(10))
        .unwrap();

    parse_results(top_docs, search_type, search_term, schema, searcher)
}

fn parse_results(
    docs: Vec<(f32, DocAddress)>,
    search_type: SearchType,
    query: String,
    schema: Schema,
    searcher: IndexReader,
) -> ReturnTypes {
    match search_type {
        SearchType::HTML => ReturnTypes::HTML(search_as_html(docs, query, schema, searcher)),
        SearchType::JSON => ReturnTypes::JSON(search_as_json(docs, query, schema, searcher)),
    }
}

pub fn search_as_json(
    docs: Vec<(f32, DocAddress)>,
    _query: String,
    schema: Schema,
    searcher: IndexReader,
) -> Json::Value {
    let mut result = Vec::new();

    for (_score, doc_address) in docs {
        let retrieved_doc = searcher.searcher().doc(doc_address).unwrap();

        result.push(schema.to_named_doc(&retrieved_doc));
    }

    serde_json::to_value(result).unwrap()
}

pub fn search_as_html(
    docs: Vec<(f32, DocAddress)>,
    query: String,
    schema: Schema,
    searcher: IndexReader,
) -> Json::Value {
    let mut result = Vec::new();

    for (_score, doc_address) in docs {
        let retrieved_doc = searcher
            .searcher()
            .doc(doc_address)
            .expect("Could not open searcher");
        let parsed_doc = schema.to_json(&retrieved_doc);

        let poem: PoemMap =
            serde_json::from_str(parsed_doc.as_str()).expect("Could not parse data");

        result.push(poem);
    }

    result = convert_to_html(result, query);

    serde_json::to_value(result).unwrap()
}

pub fn convert_to_html(mut doc: Vec<PoemMap>, query: String) -> Vec<PoemMap> {
    for poem in doc.iter_mut() {
        let newline_re = Regex::new("\n").unwrap();
        let query_re = Regex::new(format!(r"{}", query).as_str()).unwrap();

        let mut result = query_re
            .replace_all(
                poem.body.first().unwrap(),
                format!("<b>{}</b>", query).as_str(),
            )
            .to_string();

        result = newline_re.replace_all(result.as_str(), "<br>").to_string();

        poem.body = vec![result];
    }

    doc
}
