use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
pub struct OpenedFile {
    pub file_path: String,
    pub search_term: String,
    pub file_contents: String,
    pub tags: Vec<String>,
}


impl OpenedFile {
    pub fn init(file_path: &String, search_term: String, file_contents: String, tags: Option<&Vec<String>>) -> OpenedFile {
        OpenedFile{file_path: file_path.to_string(), search_term, file_contents, tags: tags.unwrap_or(&Vec::new()).to_vec()}
    }
    pub fn search_contents(&self) -> Vec<String> {
    
        let mut found_words: Vec<String> = Vec::new();
        let lines = self.file_contents.split_terminator('\n');
        for line in lines {
            if line.contains(&self.search_term) {
                let output_line: String = line.replace(&self.search_term, 
                    format!("\x1b[1;31m{}\x1b[0m", &self.search_term).as_str());
                    found_words.push(output_line);
                }        
            }
        found_words
    }
}

pub fn open_file(file_path: &String) -> String {
    let mut s: String = String::new();
    File::open(file_path).unwrap().read_to_string(&mut s).expect("Could not read to string");
    s
}
