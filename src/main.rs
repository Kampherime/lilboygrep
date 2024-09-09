mod flags;

use std::env;
use std::fs::File;
use std::io::Read;
use std::process;
//use std::time::Instant;


// Thinking of things to do for this project. Not looking at the rust docs.
// I need to open any files and check if they're valid text files
// I can use error handling and a for loop for this ^
// Then I need to search for the provided search term 


struct OpenedFile {
    search_term: String,
    file_contents: String,
}

// TODO: Print each line to the console that contains the word
// IMPROVE THE SEARCH ALGORITHM. A LOT. ITS N^2 LIKE THATS GUH
// Index the lines
// override display trait to match grep
// Color code the word
// Implement pipe operator functionality 
// implement flags: 
//    -raw => print the rust dbg format 
//    -trace => prints line numbers 
//    -:3 => counts the number of :3's in the file (just the :3's), autoapplies trace
//    -count => counts the number of them
//
impl OpenedFile {
    fn init(search_term: String, file_contents: String) -> OpenedFile {
        OpenedFile{search_term, file_contents}
    }
    // I need to find out how to check if the word is IN another word, not just on its own.
    // I did the latter w/my Go compiler
    
    // currently lists the same line twice if it has multiple things
    // need to fix.
    fn search_contents(&self) -> Vec<&str> {
        let mut found_words: Vec<&str> = Vec::new();
        let lines = self.file_contents.split_terminator('\n');
        for line in lines {
            for word in line.split_whitespace() {
                if word.contains(&self.search_term) {
                    found_words.push(line);
                }
            }        
        }
        found_words
    }
}

fn open_file(file_path: &String) -> String {
    let mut s: String = String::new();
    File::open(file_path).unwrap().read_to_string(&mut s).expect("Could not read to string");
    s
}

fn main() {
    let args: Vec<String> = env::args().collect(); //collects cli arguments 
    
    if args.len() == 1 || args.len() == 2 {
        flags::print_help();
        process::exit(0);
    }
    if args[1].contains("--") {
        flags::handle_flags(&args[1]);
    }
    let file_contents = open_file(&args[1]);
    let search_term = &args[2];
    let file = OpenedFile::init(search_term.to_string(), file_contents);
    dbg!(file.search_contents()/*.len()*/);
}


