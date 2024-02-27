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
    columns: Vec<TableContent>,
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

    fn add_col(&mut self, content: TableContent) -> &Table {
        self.columns.push(content);
        self
    }

    fn format(&self) -> String {
        match (self.mode.unwrap_or(Tables::Table)) {
            Tables::List => {
                let mut output: String = String::new();
                let output_cols: Vec<Vec<String>> = Vec::new();
                let mut height = 0;
                for col in self.columns {
                    let mut min_width = col.head.unwrap_or(String::new()).len();
                    let tempcols: Vec<String> = Vec::new();
                    tempcols.push(col.head.unwrap_or(String::new()));
                    height = 2;
                    for content in col.content {
                        if content.len() > min_width {
                            min_width = content.len();
                        }
                        tempcols.push(format!("{}", content));
                        height += 1;
                    }
                    tempcols.push((0..min_width + 2).map(|_| "-").collect::<String>());
                    output_cols.push(tempcols);
                }

                let mut n = 0;

                for columns in output_cols {
                    // Generate head row
                    while n < output_cols.len() - 1 {
                        output = format!("{}|{}", output, output_cols[n][0]);
                        n += 1;
                    }
                    n = 0;
                    output = format!("{}\n", output);
                    while n < output_cols.len() - 1 {
                        output = format!("{}|{}", output, output_cols[n][output_cols[n].len() - 1]);
                        n += 1;
                    }
                    n = 0;
                    for col in columns {
                        if n == 0 {
                            n += 1;
                            continue;
                        }

                        if n == columns.len() - 1 {
                            continue;
                        }

                        while n < output_cols.len() {
                            output
                        }
                    } 
                } 
                output
            },
            Tables::Table => {

            }
        }
    }

    fn display() {
        
    }
}

fn main() {
    let parser = ArgParser::new();
    println!("Hello world! {:?}", &parser);
}
