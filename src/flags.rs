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
#[allow(dead_code)]
pub fn handle_flags(cli_arg: &String) {
    let _ = match cli_arg.as_str() {
        "--trace" => todo!(),
        "--count" => todo!(),
        "--raw" => todo!(),
        "--regex" => todo!(),
        _ => {
            panic!("Flag does not exist.")
        },
    };
}

/// counts the number of times the expression shows up in the file
#[allow(dead_code)]
pub fn count_flag() -> i32 {
    
    5 
}
/// Shows the line numbers in the output (could also be verbose)
#[allow(dead_code)]
pub fn trace_flag() /*need return type*/ {
}

/// Prints in dbg!(), that's it.
#[allow(dead_code)]
pub fn raw_flag() /*need return type (probably none) */{ 
}
