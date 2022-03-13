use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let my_path = &args[0];
    println!("My path is {}.", my_path);
    let param_count = args.len() - 1;
    println!("I got {:?} arguments: {:?}.", param_count, &args[1..]);

    //println!("Hello, world!");
}
