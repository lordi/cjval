use jsonschema::{Draft, JSONSchema};

use serde_json::Value;
use std::collections::HashMap;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    cityjson_file: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();

    //-- fetch the CityJSON data file
    let s1 = std::fs::read_to_string(&args.cityjson_file).expect("Couldn't read CityJSON file");
    let j = serde_json::from_str(&s1).unwrap();

    //-- fetch the correct schema
    let schema_str = include_str!("../schemas/cityjson.min.schema.json");
    let schema = serde_json::from_str(schema_str).unwrap();
    // if is_cityjson_file(&j) == false {
    // println!("OUPSIE");
    // }
    let v = get_version_cityjson(&j);
    if v == 10 {
        println!("version {:?}", v);
    } else if v == 11 {
        println!("version {:?}", v);
    } else {
        println!("VERSION NOT SUPPORTED");
    }

    let compiled = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)
        .expect("A valid schema");
    let result = compiled.validate(&j);
    if let Err(errors) = result {
        for error in errors {
            println!("Validation error: {}", error);
            println!("Instance path: {}", error.instance_path);
        }
    } else {
        println!("valid 👍");
    }
}

fn is_cityjson_file(j: &Value) -> bool {
    if j["type"] == "CityJSON" {
        true
    } else {
        false
    }
}

fn get_version_cityjson(j: &Value) -> i8 {
    if j["version"] == "1.1" {
        11
    } else if j["version"] == 1.0 {
        10
    } else {
        -1
    }
}

fn validate_no_duplicate_vertices(j: &Value) -> bool {
    let mut valid = true;
    let verts = j
        .get("vertices")
        .expect("no vertices")
        .as_array()
        .expect("not an array");
    // use all vertices as keys in a hashmap
    let mut uniques = HashMap::new();
    for i in 0..verts.len() {
        let vert = verts[i].as_array().unwrap();
        let arr = [
            vert[0].to_string(),
            vert[1].to_string(),
            vert[2].to_string(),
        ];
        if !uniques.contains_key(&arr) {
            uniques.insert(arr, i);
        } else {
            // duplicate found!
            let other = uniques.get(&arr).unwrap();
            valid = false;
            // // feedback
            // plog!("");
            // plog!("Duplicate Vertex Error");
            // plog!("  L indices : vertices[{}] == vertices[{}]", other, i);
            // plog!("  L vertex  : [{}, {}, {}]", arr[0], arr[1], arr[2]);
        }
    }
    return valid;
}