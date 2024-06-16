use std::{fs, time::UNIX_EPOCH};

#[derive(Debug)]
struct ArgParser {
  list: Option<bool>,
  all: Option<bool>,
  folder_classification: Option<bool>,
  sortmodified: Option<bool>,
  simple: Option<bool>
}

impl ArgParser {
  fn new() -> ArgParser {
    let mut parser = ArgParser { list: None, all: None, folder_classification: None, sortmodified: None, simple: None };
    let args: Vec<String> = std::env::args().map(|y| y.to_owned()).collect();
    let mut n = 0;

    for arg in args {
      if n == 0 {
        n = n + 1;
        continue;
      }
      match arg.as_str() {
        "-l" => {
          parser.list = Some(true);
        },
        "-s" => {
          parser.simple = Some(true);
        }
        "-a" => {
          parser.all = Some(true);
        },
        "-F" => {
          parser.folder_classification = Some(true);
        },
        "-t" => {
          parser.sortmodified = Some(true);
        },
        _ => {
          panic!("Unknown argument specified")
        }
      }
    }

    parser
  }
}

struct TableContent {
  head: Option<String>,
  content: Vec<String>,
  min_width: u32
}

impl TableContent {
  fn new_from_head(head: String) -> TableContent {
    let min_width = {
      if (head.trim().len() + 2) as u32 > 9 {
        if (head.trim().len() + 2) as u32 > 25 {
          if head.as_str().to_owned().split(".").last().unwrap_or("").trim().is_empty() {
            25
          }
          else
          {
            head.as_str().to_owned().split(".").last().unwrap().trim().len() as u32 + 25
          }
        }
        else
        {
          (head.trim().len() + 2) as u32
        }
      }
      else
      {
        9
      }
    };

    TableContent {
      head: Some(head.as_str().to_owned()),
      content: vec![],
      min_width
    }
  }

  fn add(&mut self, content: String) {
    self.content.push(content.trim().to_owned());
    if (content.trim().len() + 2) as u32 > self.min_width {
      self.min_width = {
        if (content.trim().len() + 2) as u32 > 9 {
          if (content.trim().len() + 2) as u32 > 25 {
            if content.as_str().to_owned().split(".").last().unwrap_or("").trim().is_empty() {
              25
            }
            else
            {
              content.as_str().to_owned().split(".").last().unwrap().trim().len() as u32 + 25
            }
          }
          else
          {
            (content.trim().len() + 2) as u32
          }
        }
        else
        {
          9
        }
      };
    }
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

  fn new(mode: Tables, columns: Vec<TableContent>) -> Table {
    return Table {
      mode: Some(mode),
      columns
    }
  }

  fn pad(instr: String, min_width: u32) -> String {
    let mut str = instr.as_str().to_owned();
    let end = instr.split(".").last().unwrap_or("");
    // Cap the string at 25 characters with an ellipsis if necessary
    if str.len() > 25 {
      str.truncate(22); // Truncate to 22 characters to leave room for the ellipsis
      if end.is_empty() {
        str.push_str("...");
      } else {
        str.truncate(18);
        str.push_str(format!("[..].{}", end).as_str());
      }
    }

    // Calculate the required padding
    if (str.len() + 1) as u32 >= min_width {
      format!(" {}", str)
    } else {
      format!(
        " {}{}",
        str,
        (0..(min_width - (str.len() + 1) as u32))
          .map(|_| " ")
          .collect::<String>()
      )
    }
  }

  fn as_table(cols: Vec<TableContent>) -> String {
    let mut ret = cols.iter().map(|x| Table::pad(x.head.clone().unwrap_or(String::from("unknown")), x.min_width)).collect::<Vec<String>>().join("|");
    let full_width = ret.len();
    ret = format!("{}\n{}", ret, (0..full_width).map(|_| "-").collect::<String>());

    let mut n = 0;

    loop {
      if n == 0 && n >= cols.len() {break;}
      if n >= cols[0].content.len() {break;}
      let mut temp_vec: Vec<String> = Vec::new();

      for content in &cols {
        temp_vec.push(Table::pad(content.content[n].as_str().to_owned(), content.min_width));
      }

      ret = format!("{}\n{}", ret, temp_vec.join("|"));
      n = n + 1;
    }

    return ret;
  }

  fn as_list(cols: Vec<TableContent>, simple: bool) -> String {
    let mut ret: String = String::new();
    for content in &cols {
      let head = content.head.clone().unwrap_or(String::from("unknown"));
      if ret.len() == 0 {
        ret = format!("{}", head);
      } else {
        ret = format!("{}\n\n{}", ret, head);
      }
      ret = format!("{}\n{}", ret, (0..((head.len() + 3) as u32)).map(|_| "-").collect::<String>());
      let mut first_content: bool = false;
      for c in &content.content {
        if !first_content || simple {
          ret = format!("{}\n{}", ret, c);
          first_content = true;
        } else {
          ret = format!("{}  {}", ret, c);
        }
      }
    }

    return ret;
  }

  fn to_readable(self, simple: bool) -> String {
    let mode = self.mode.unwrap_or(Tables::Table);

    let return_product: String = match mode {
      Tables::List => {
        Table::as_list(self.columns, simple)
      },
      Tables::Table => {
        Table::as_table(self.columns)
      }
    };

    return return_product;
  }
}

#[derive(PartialEq, Eq, Ord, PartialOrd)]
enum FileFolderType {
  HiddenFolder,
  Folder,
  HiddenFile,
  File,
}

struct FileFolder {
  name: String,
  fftype: FileFolderType,
  modified: u64,
}

impl FileFolder {
  fn new(sortmodify: bool) -> Vec<FileFolder> {
    let mut entries: Vec<FileFolder> = Vec::new();
    let current_dir = match fs::read_dir(".") {
      Ok(x) => x,
      Err(e) => panic!("{} {}", "There was an error reading the directory!", e),
    };

    for entry in current_dir {
      let entry = match entry {
        Ok(x) => x,
        Err(e) => panic!("{} {}", "There was an error getting an entry", e),
      };
      let meta = match entry.metadata() {
        Ok(x) => x,
        Err(e) => panic!("{} {}", "There was an error reading the metadata of an entry", e),
      };

      let name = entry.file_name().into_string().unwrap_or_default();

      let hidden = name.starts_with(".");
      let modified = match meta.modified() {
        Ok(x) => x.duration_since(UNIX_EPOCH).unwrap().as_secs(),
        Err(e) => panic!("{} {}", "There was an error reading the modified time of an entry", e),
      };

      let fftype = if meta.is_dir() {
        if hidden {
          FileFolderType::HiddenFolder
        } else {
          FileFolderType::Folder
        }
      } else {
        if hidden {
          FileFolderType::HiddenFile
        } else {
          FileFolderType::File
        }
      };

      entries.push(FileFolder { name, fftype, modified })
    }

    if sortmodify {
      entries.sort_by(|a, b| a.modified.cmp(&b.modified));
    } else {
      entries.sort_by(|a, b| a.fftype.cmp(&b.fftype));
    }

    return entries;
  }
}

fn main() {
  let parser = ArgParser::new();
  let list = parser.list.unwrap_or(false);
  let all = parser.all.unwrap_or(false);
  let ff = FileFolder::new(parser.sortmodified.unwrap_or(false));
  let markfolder = parser.folder_classification.unwrap_or(true);
  let simple = parser.simple.unwrap_or(false);

  let mut content: TableContent = TableContent::new_from_head(Table::pad(String::from("Entries"), 9));
  let mut modifytimes: TableContent = TableContent::new_from_head(Table::pad(String::from("Modified"), 10));

  for entry in ff {
    if all {
      if markfolder && (entry.fftype.eq(&FileFolderType::Folder) || entry.fftype.eq(&FileFolderType::HiddenFolder)) {
        content.add(format!("{}/", Table::pad(entry.name.as_str().to_owned(), (entry.name.len() + 1) as u32)));
      } else {
        content.add(Table::pad(entry.name.as_str().to_owned(), (entry.name.len() + 2) as u32));
      }
      modifytimes.add(entry.modified.to_string());
    } else if !entry.fftype.eq(&FileFolderType::HiddenFile) && !entry.fftype.eq(&FileFolderType::HiddenFolder) {
      if markfolder && entry.fftype.eq(&FileFolderType::Folder) {
        content.add(format!("{}/", Table::pad(entry.name.as_str().to_owned(), (entry.name.len() + 1) as u32)));
      } else {
        content.add(Table::pad(entry.name.as_str().to_owned(), (entry.name.len() + 2) as u32));
      }
      modifytimes.add(entry.modified.to_string());
    }
  }

  if list {
    print!("{}", Table::new(Tables::List, vec![content]).to_readable(false));
  } else if !simple {
    print!("{}", Table::new(Tables::Table, vec![content, modifytimes]).to_readable(false));
  } else {
    print!("{}", Table::new(Tables::List, vec![content]).to_readable(simple));
  }
}
