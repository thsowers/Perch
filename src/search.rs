use crate::build_search_index::{build_schema, create_index};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;

pub fn search(search_term: String) -> tantivy::Result<()> {
    // Setup fields
    let schema = build_schema();
    let index = create_index(&schema);

    let title = schema.get_field("title").unwrap();
    let body = schema.get_field("body").unwrap();
    // Load search index
    index.load_searchers()?;

    // Acquire a new worker
    let searcher = index.searcher();

    // Setup query
    let query_parser = QueryParser::for_index(&index, vec![title, body]);
    let query = query_parser.parse_query(&search_term)?;

    // Perform our query, and return the top ten results
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

    for (_score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;

        // TODO: Pretty print results
        println!("{}", schema.to_json(&retrieved_doc));
    }

    Ok(())
}
