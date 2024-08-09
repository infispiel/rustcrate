use serde_json::{Value,json};
use chrono::{self, Utc};

const ROCRATE_SPEC : &str = "https://w3id.org/ro/crate/1.1";

fn main() {
    println!("Hello, world!");
    generate_boilerplate_context(generate_root("asd", "fds", get_time_right_now()));
}

/// Returns the current datetime in UTC.
/// 
/// # Output
/// chrono::DateTime<Utc>
fn get_time_right_now() -> chrono::DateTime<Utc> {
    chrono::prelude::Utc::now()
}

// accepts time in UTC; convert before calling
fn generate_root(name: &str, desc: &str, date_published: chrono::DateTime<Utc>) -> Value {
    // todo: add license parameter
    json!({
        "@id": "./",
        "@type": "Dataset",
        "hasPart": [

        ],
        "name": name,
        "description": desc,
        "datePublished": date_published,
        //"license": {}
    })
}

fn generate_boilerplate_context(root_context: Value) {
    let rocrate_metadata_descriptor: Value = json!({
        "@id": "ro-crate-metadata.json",
        "@type": "CreativeWork",
        "conformsTo": {"@id": ROCRATE_SPEC},
        "about": {"@id": "./"}
    });

    let context: Value = json!({
        "@context": format!("{ROCRATE_SPEC}/context"),
        "@graph": [
            rocrate_metadata_descriptor,
            root_context
        ]
    });

    
    println!("{}", context);
}