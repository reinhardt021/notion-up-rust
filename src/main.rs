use std::env;
use std::fs::File;
//use std::io::{self, BufReader};
use std::io::prelude::*;
use std::path::Path;
use dotenv::dotenv;
use std::collections::HashMap;
//use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

fn main() {
    dotenv().ok();

    let token_key = "BEARER_TOKEN";
    let token = dotenv::var(token_key).unwrap();
    println!("{}: {}",  token_key, token);

    let notion_key = "NOTION_VERSION";
    let notion_version = dotenv::var(notion_key).unwrap();
    println!("{}: {}",  notion_key, notion_version);
    
    let database_key = "DATABASE_ID";
    let database_id = dotenv::var(database_key).unwrap();
    println!("{}: {}",  database_key, database_id);

    let args: Vec<String> = env::args().collect();
    let command_path = &args[0];
    println!("My path is {}.", command_path);
    let param_count = args.len() - 1;
    println!("I got {:?} arguments: {:?}.", param_count, &args[1..]);
    let file_path = &args[1];

    let path = Path::new(file_path);
    let display = path.display();
    
    let mut file = match File::open(&path) {
        Err(why) => panic!("Could not open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Err(why) => panic!("Could not read {}: {}", display, why),
        Ok(_) => transform_pixels_for_notion(data),
    };

}

//#[derive(Serialize, Deserialize)]
//struct Tag {
    //entries: Vec<String>,
//}

//#[derive(Serialize, Deserialize)]
//struct Entry {
    //isHighlighted: String,
    //notes: String,
    //tags: Vec<Tag>,
    //value: i32,
//}

//#[derive(Serialize, Deserialize)]
//struct Pixel {
    //date: String,
    //entries: Vec<Entry>,
//}

fn transform_pixels_for_notion(json_string: String) -> Result<()> {
    println!("Read file now parsing...");
    let my_json: Vec<HashMap<String, Value>> = serde_json::from_str(&json_string)?;
    //let my_json: Vec<Pixel> = serde_json::from_str(&json_string)?;
    for item in my_json.iter() {
        //println!("> {:?}", item);
        println!("> {:?}", item.keys());
        let curr_date = &item["date"];
        println!("> {}", curr_date);
        // TODO: loop through entries to build a new JSON object
        println!("> {:?}", item.get("entries").expect("entries").unwrap());
        //let entries: Vec<HashMap<String, Value>> = serde_json::value::Value::deserialize(&item["entries"]);
        //for entry in entries.iter() {
            //println!("> {:?}", entry);
        //} 
        // TODO: use new entry to do an API call to CREATE A PAGE on NOTION
    }
    //println!("{:?}", my_json);

    Ok(())
}
