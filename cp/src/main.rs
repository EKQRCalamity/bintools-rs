pub fn read_input(prompt: &str) -> String {
    use std::io::{self, Write};
    let mut buffer: String = String::new();
    print!("{} ", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_owned()
}

#[derive(Debug)]
struct ArgParser {
  from: String,
  to_original: String,
  to: String,
  force: Option<bool>
}

impl ArgParser {
  fn new() -> ArgParser {
    let args: Vec<String> = std::env::args().map(|y| y.to_owned()).collect();
    let mut parser = ArgParser { from: String::new(), to: String::new(), to_original: String::new(), force: None };
    let mut n: i8 = 0;
    for arg in args {
      if n == 0 {
        n = n + 1;
        continue;
      }
      match arg.as_str() {
        "-f" => parser.force = Some(true),
        _ => {
          if n == 1 {
            parser.from = arg;
            n = n + 1;
          } else if n == 2 {
            parser.to = arg.to_owned();
            parser.to_original = arg;
            n = n + 1;
          } else {
            println!("Too many arguments supplied. Continuing with first.");
            break;
          }
        }
      }
    }
    if parser.to.ends_with("/") || parser.to.ends_with("\\") {
      let file_forward_slash: String = parser.from.split("/").map(|x| x.to_string()).last().unwrap_or(parser.from.as_str().to_string());
      let file_backward_slash: String = parser.from.split("\\").map(|x| x.to_string()).last().unwrap_or(parser.from.as_str().to_string());

      let filename: String = if file_forward_slash.len() > file_backward_slash.len() { file_backward_slash } else { file_forward_slash };

      parser.to = format!("{}{}", parser.to, filename);
    }
    return parser;
  }
}

fn main() {
  let parser = ArgParser::new();
  let force = parser.force.unwrap_or(false);
  if parser.to == String::new() || parser.from == String::new() {
    eprintln!("Copy paths not specified.");
    std::process::exit(1);
  }
  if std::path::Path::new(&parser.to).exists() && !force {
    let input = read_input(format!("Are you sure you want to overwrite the file? ({}) [Y/n]", &parser.to).as_str());
    if input.to_lowercase().as_str() != "y" {
      eprintln!("Operation aborted.");
      std::process::exit(1);
    }
  }

  if !std::path::Path::new(&parser.to_original).exists() && (parser.to_original.ends_with("/") || parser.to_original.ends_with("\\")) {
    match std::fs::create_dir_all(&parser.to_original) {
      Ok(_) => {
      },
      Err(e) => {
        eprintln!("Error occured while creating target directory: {}", e);
        std::process::exit(1);
      }
    };
  }

  match std::fs::copy(parser.from, parser.to) {
    Ok(_) => {
      println!("Files copied.");
    },
    Err(x) => {
      println!("Could not copy, error occurred: {}", x);
    }
  };
}
