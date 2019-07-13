use crate::build_search_index::{build_schema, create_index};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;

pub fn search(search_term: String) -> tantivy::Result<(Vec<String>)> {
    // Setup fields
    let schema = build_schema();
    let index = create_index(&schema);
    let mut result= Vec::new();

    let title = schema.get_field("title").unwrap();
    let body = schema.get_field("body").unwrap();

    // Setup Reader
    let reader = index.reader()?;

    // Acquire a new worker
    let searcher = reader.searcher();

    // Setup query
    let query_parser = QueryParser::for_index(&index, vec![title, body]);
    let query = query_parser.parse_query(&search_term)?;

    // Perform our query, and return the top ten results
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

    for (_score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;

        result.push(schema.to_json(&retrieved_doc));
    }

    Ok(result)
}
