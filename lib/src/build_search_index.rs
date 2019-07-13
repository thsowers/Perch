use crate::models::{Poems};
use std::fs::create_dir;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use tantivy::directory::MmapDirectory;
use tantivy::schema::*;
use tantivy::Index;

pub fn write_persistent_index() -> tantivy::Result<()> {
    // Create index path
    let path = Path::new("./db");
    let _index_path = create_dir(path).unwrap();

    // Build fields
    let schema = build_schema();

    // Create Index & Writer
    let index = create_index(&schema);
    let mut index_writer = index.writer(50_000_000)?;

    let title = schema.get_field("title").unwrap();
    let author = schema.get_field("author").unwrap();
    let body = schema.get_field("body").unwrap();

    // Deserialize Poem JSON
    let mut s = String::new();
    let _data = File::open("poems.json")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    let deserialized_poems: Poems = serde_json::from_str(&s).expect("error while reading json");

    // Add poems into index
    for poem in deserialized_poems.poems {
        index_writer.add_document(doc!(
        title => poem.title.to_string(),
        author => poem.author.to_string(),
        body => poem.text.join("\n").replace("\n", " ")
        ));
    }

    // Finish processing documents in the queue, flush the current index to disk
    // This call is blocking.
    index_writer.commit().unwrap();

    Ok(())
}

pub fn create_index(schema: &Schema) -> Index {
    Index::open_or_create(MmapDirectory::open("./db").unwrap(), schema.clone()).unwrap()
}

pub fn build_schema() -> Schema {
    // Setup Schema
    let mut schema_builder = Schema::builder();

    // Build fields
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("author", TEXT | STORED);
    schema_builder.add_text_field("body", TEXT | STORED);
    schema_builder.build()
}
