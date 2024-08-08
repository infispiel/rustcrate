use std::io;
use serde;
use serde_json;
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
    return chrono::prelude::Utc::now();
}

// accepts time in UTC; convert before calling
fn generate_root(name: &str, desc: &str, date_published: chrono::DateTime<Utc>) -> serde_json::Value {
    // todo: add license parameter
    let rocrate_root: serde_json::Value = serde_json::json!({
        "@id": "./",
        "@type": "Dataset",
        "hasPart": [

        ],
        "name": name,
        "description": desc,
        "datePublished": date_published,
        //"license": {}
    });

    return rocrate_root
}

fn generate_boilerplate_context(root_context: serde_json::Value) {
    let rocrate_metadata_descriptor: serde_json::Value = serde_json::json!({
        "@id": "ro-crate-metadata.json",
        "@type": "CreativeWork",
        "conformsTo": {"@id": ROCRATE_SPEC},
        "about": {"@id": "./"}
    });

    let context: serde_json::Value = serde_json::json!({
        "@context": format!("{ROCRATE_SPEC}/context"),
        "@graph": [
            rocrate_metadata_descriptor,
            root_context
        ]
    });

    
    println!("{}", context.to_string());
}