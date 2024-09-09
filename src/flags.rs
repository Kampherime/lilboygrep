use crate::filehandling::*;
use std::process;


pub fn print_help() {
    print!(
    "LilBoyGrep USAGE: CMD + <OPTIONS> + <file> + <search_term>\nKatie's implementation of grep! Probably sucks.\n
    Flag options:\n\t
    --trace: Prints the line numbers of each line found.\n\t
    --count: Prints the amount of times that the existing expression appears.\n\t
    --raw: Prints the output in the default rust dbg! format (idk why you'd want to? but you can!).\n
    TO BE IMPLEMENTED:\n\t
    --regex: Matches the provided regex expression.\n
    And a secret flag :)\n\n")
}

/// This is intended to be used IF THE FLAG is found in cli_args[1], then
/// the arg will be passed to this function and handled.
/// This function might not work depending on how rust match statements work...
//#[allow(dead_code)]
pub fn handle_flags(cli_args: &Vec<String>) {
    
    let mut flags: Vec<String> = Vec::new();
    let mut arg_count = 0;
    for arg in cli_args {
        if arg.contains("--") {
            arg_count += 1;
            flags.push(arg.to_string());      
        }
    }
    if arg_count >= 2 {
        println!("Only one argument supported at this time.
            This feature will be added eventually... 
            when I get to it.");
        process::exit(1);
    }
    //TO FIX: SUPPORT FOR ONLY ONE FLAG
    let file_contents = open_file(&cli_args[2]);
    let search_term = &cli_args[3];
    let file = OpenedFile::init(&cli_args[2], search_term.to_string(), file_contents, Some(&flags));

    for flag in flags {

    let _ = match flag.as_str() {
        "--trace" => todo!(),
        "--count" => count_flag(&file),
        "--raw" => raw_flag(&file),
        "--regex" => todo!(),
        _ => {
            panic!("Flag does not exist.")
        },
    };
    }
}

/// counts the number of times the expression shows up in the file
#[allow(dead_code)]
pub fn count_flag(file: &OpenedFile) {
    println!("Amount of times that {} showed up in {}: {}", file.search_term, file.file_path, file.search_contents().len());
}
/// Shows the line numbers in the output (could also be verbose)
#[allow(dead_code)]
pub fn trace_flag() /*need return type*/ {
}

/// Prints in dbg!(), that's it.
#[allow(dead_code)]
pub fn raw_flag(file: &OpenedFile) /*need return type (probably none) */{ 
    dbg!("{}", file.search_contents());
}

/*#[allow(dead_code)]
pub fn handle_file(file: OpenedFile) -> Vec<String> {
}*/
