mod flags;
mod filehandling;

use std::env;
use std::process;
//use std::time::Instant; for timing

// Thinking of things to do for this project. Not looking at the rust docs.
// I need to open any files and check if they're valid text files
// I can use error handling and a for loop for this ^
// Then I need to search for the provided search term 


// TODO: Print each line to the console that contains the word
// IMPROVE THE SEARCH ALGORITHM. A LOT. ITS N^2 LIKE THATS GUH
// Index the lines
// override display trait to match grep
// Color code the word
// Implement pipe operator functionality 
// implement flags: 
//    --raw => print the rust dbg format 
//    --trace => prints line numbers 
//    --:3 => counts the number of :3's in the file (just the :3's), autoapplies trace
//    --count => counts the number of them
//    --regex => Utilizes regex parsing 


///Main potentially defers to a flag for content searching depending on the flag used.
fn main() {
    let args: Vec<String> = env::args().collect(); //collects cli arguments 
    
    if args.len() == 1 || args.len() == 2 {
        flags::print_help();
        process::exit(0);
    }
    //TO FIX: SUPPORT FOR ONLY ONE FLAG
    if args[1].contains("--") {
        // I want to add capability to process all of the tags at once...
        flags::handle_flags(&args);
        process::exit(0);
    }
    let file_contents = filehandling::open_file(&args[1]);
    let search_term = &args[2];
    let file = filehandling::OpenedFile::init(&args[1], search_term.to_string(), file_contents, None);
    for line in file.search_contents() {
        println!("{}", line);
    }
    //dbg!(file.search_contents()/*.len()*/);
}


