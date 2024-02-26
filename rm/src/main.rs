
#[derive(Debug)]
struct ArgParser {
    paths: Vec<String>
}

impl ArgParser {
    fn new() -> ArgParser {
        let args: Vec<String> = std::env::args().map(|y| y.to_owned()).collect();
        let mut parser = ArgParser { paths: vec![] }; 
        let mut first: bool = true;
        for arg in args {
            if first {
                first = false;
                continue;
            }
            parser.paths.push(arg);
        }
        parser
    }    
}

pub fn read_input(prompt: &str) -> String {
    use std::io::{self, Write};
    let mut buffer: String = String::new();
    print!("{} ", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_owned()
}

fn main() {
    let parser = ArgParser::new();
     
    if parser.paths.is_empty() {
        println!("No files or folders were specified.");
    }
    
    for path in parser.paths {
        let as_path = std::path::Path::new(&path);
        if !as_path.exists() {
            println!("Skipping {} file does not exist.", &path);
        } else {
            if as_path.is_dir() {
                let response = read_input(
                    format!(
                        "Are you sure you want to delete this folder along with its contents? ({}) [Y/N]", &path
                        ).as_str()
                    );
                if response.to_lowercase() == "y" {
                    match std::fs::remove_dir_all(&as_path) {
                        Ok(_) => {
                            println!("Folder and contents removed! ({})", &path);
                        },
                        Err(e) => {
                            println!("Couldn't remove folder and content. ({}) {:?}", &path, e);
                        }
                    }
                }
            } else {
                match std::fs::remove_file(&as_path) {
                    Ok(_) => {
                        println!("File removed! ({})", &path);
                    },
                    Err(e) => {
                        println!("Couldn't remove file. ({}) ({:?})", &path, e);
                    }
                }
            }
        }
    }
}
