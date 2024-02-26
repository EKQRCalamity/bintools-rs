#[derive(Debug)]
struct ArgParser {
    list: Option<bool>,
    all: Option<bool>,
    folder_classification: Option<bool>,
    sortmodfied: Option<bool>, 
}

impl ArgParser {
    fn new() -> ArgParser {
        let mut parser = ArgParser { list: None, all: None, folder_classification: None, sortmodfied: None };
        let args: Vec<String> = std::env::args().map(|y| y.to_owned()).collect();
        let mut n = 0;

        for arg in args {
            if n == 0 {
                n = n + 1;
                continue;
            }
            if n == 1 {
                match arg.as_str() {
                    "-l" => {
                        parser.list = Some(true);
                    },
                    "-a" => {
                        parser.all = Some(true);
                    },
                    "-F" => {
                        parser.folder_classification = Some(true);
                    },
                    "-t" => {
                        parser.sortmodfied = Some(true);
                    },
                    _ => {
                        panic!("Unknown argument specified")
                    }
                }
                n = n + 1;
            } else {
                panic!("More than one argument was specified.")
            }
        }

        parser
    }
}

struct TableContent {
    head: Option<String>,
    content: Vec<String>
}

impl TableContent {
    fn new_from_head(head: String) -> TableContent {
        TableContent {
            head: Some(head),
            content: vec![]
        }
    }

    fn new() -> TableContent {
        TableContent {
            head: None,
            content: vec![]
        }
    }

    fn set_head(&mut self, head: String) -> &TableContent {
        self.head = Some(head);
        self
    }

    fn add(&mut self, content: String) -> &TableContent {
        self.content.push(content);
        self
    }
}


enum Tables {
    List,
    Table
}

struct Table {
    mode: Option<Tables>,
    rows: Vec<TableContent>,
}

impl Table {
    fn new() -> Table {
        Table {
            mode: None,
            rows: vec![],
        }
    }

    fn list() -> Table {
        Table {
            mode: Some(Tables::List),
            rows: vec![]
        }
    }

    fn add_row(&mut self, content: TableContent) -> &Table {
        self.rows.push(content);
        self
    }

    fn display() {
        
    }
}

fn main() {
    let parser = ArgParser::new();
    println!("Hello world! {:?}", &parser);
}
