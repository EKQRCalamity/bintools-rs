
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

fn main() {
    let parser = ArgParser::new();

    if parser.paths.is_empty() {
        println!("No files were specified.")
    }
    for path in parser.paths {
        let as_path = std::path::Path::new(&path);
        if as_path.exists() {
            println!("Skipping {} file already exists", &path);
        } else {
            if as_path.is_dir() || path.ends_with("/") {
                let mut path = path.clone();
                path.truncate(path.len());
                match std::fs::create_dir_all(&as_path) {
                    Ok(_) => {
                        println!("Folder at {} created", &path);
                    },
                    Err(e) => {
                        println!("Couldn't create folder at {} | {:?}", &path, e);
                    }
                }
            } else {
                 match std::fs::File::create(&as_path) {
                    Ok(_) => {
                        println!("File {} created.", &path);
                    },
                    Err(e) => {
                        println!("Couldn't create file {} | {:?}", &path, e);
                    }
                }
            }
        }
    }
}
