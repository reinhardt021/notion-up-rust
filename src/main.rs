use std::env;
use std::fs::File;
//use std::io::{self, BufReader};
use std::io::prelude::*;
use std::path::Path;
use dotenv::dotenv;
use std::collections::HashMap;
use serde_json::{Result, Value};

// // x01: This doesn't seem to work right now >> will try alternative
//fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    //let file  = File::open(filename)?;
    //Ok(io::BufReader::new(file).lines())
//}

fn main() {
    dotenv().ok();
    //for (key, value) in env::vars() {
        //println!("{}: {}", key, value);
    //}
    let token_key = "BEARER_TOKEN";
    let token = dotenv::var(token_key).unwrap();
    println!("{}: {}",  token_key, token);

    let notion_key = "NOTION_VERSION";
    let notion_version = dotenv::var(notion_key).unwrap();
    println!("{}: {}",  notion_key, notion_version);
    
    let database_key = "DATABASE_ID";
    let database_id = dotenv::var(database_key).unwrap();
    println!("{}: {}",  database_key, database_id);

    //println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    let command_path = &args[0];
    println!("My path is {}.", command_path);
    let param_count = args.len() - 1;
    println!("I got {:?} arguments: {:?}.", param_count, &args[1..]);
    let file_path = &args[1];

    // // x01: This doesn't seem to work right now >> will try alternative
    //if let Ok(lines) = read_lines(&args[1]) {
         // // Consumes the iterator (lines), returns an (Optional) String
        //for line in lines {
            //if let Ok(json_line) = line {
                //println!("{}", json_line);
            //}
        //}
    //}

    // x02
    let path = Path::new(file_path);
    let display = path.display();
    
    let mut file = match File::open(&path) {
        Err(why) => panic!("Could not open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Could not read {}: {}", display, why),
        Ok(_) => transform_pixels_for_notion(s),
        //Ok(_) => println!("Was able to read file"),
        //Ok(_) => println!("{} contains: \n{}", display, s),
    };

}

fn transform_pixels_for_notion(json_string: String) -> Result<()> {
    println!("Read file now parsing...");
    let my_json: Vec<HashMap<String, Value>> = serde_json::from_str(&json_string)?;
    for item in my_json.iter() {
        //println!("> {}", item);
        //println!("> {}", item.keys());
        println!("> {:?}", item);
        println!("> {:?}", item.keys());
    }
    //println!("{:?}", my_json);

    Ok(())
}
