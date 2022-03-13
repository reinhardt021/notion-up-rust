use std::env;
use std::fs::File;
//use std::io::{self, BufReader};
use std::io::prelude::*;
use std::path::Path;

fn main() {
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
        Ok(_) => println!("{} contains: \n{}", display, s),
    };

}

// // x01: This doesn't seem to work right now >> will try alternative
//fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    //let file  = File::open(filename)?;
    //Ok(io::BufReader::new(file).lines())
//}
