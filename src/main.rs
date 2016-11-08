#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{self, Path};
use std::str::FromStr;
use std::time;

use errors::{Error, Result};
use token::TokenStore;

mod errors;
mod token;
mod utils;

enum AddType {
    Text,
    File,
}

impl FromStr for AddType {
    type Err = Error;
    fn from_str(s: &str) -> Result<AddType> {
        match s {
            "TXT" => Ok(AddType::Text),
            "FIL" => Ok(AddType::File),
            _ => Err(Error::new("unknown add type")),
        }
    }
}

#[derive(Debug)]
enum QueryType {
    Add,
    Buffer,
    File,
}

impl FromStr for QueryType {
    type Err = Error;
    fn from_str(s: &str) -> Result<QueryType> {
        match s {
            "ADD" => Ok(QueryType::Add),
            "BUF" => Ok(QueryType::Buffer),
            "FIL" => Ok(QueryType::File),
            _ => Err(Error::new("unknown query type")),
        }
    }
}

struct Manager {
    file_map: HashMap<String, time::SystemTime>,
    buf: String,
    store: TokenStore,
}

impl Manager {
    fn new() -> Manager {
        Manager {
            file_map: HashMap::new(),
            buf: String::new(),
            store: TokenStore::new(),
        }
    }

    // Frame:
    // ADD TXT text
    //     FIL filename
    // BUF query
    // FIL query
    fn process(&mut self) -> Result<()> {
        self.buf.clear();
        try!(io::stdin().read_line(&mut self.buf));
        if self.buf.len() < 3 {
            return Err(Error::new("wrong frame format"));
        }
        let request_type = try!(self.buf[0..3].parse::<QueryType>());
        let data = &self.buf[3..].trim();

        match request_type {
            QueryType::Add => {
                if data.len() < 3 {
                    return Err(Error::new("wrong frame format"));
                }
                let add_type = try!(data[0..3].parse::<AddType>());
                try!(Self::add_content(&mut self.file_map, &mut self.store, add_type, &data[3..]));
            }
            QueryType::Buffer => try!(self.query_buffer(data)),
            QueryType::File => try!(self.list_file(data)),
        }
        Ok(())
    }

    fn query_buffer(&self, q: &str) -> Result<()> {
        if q.is_empty() {
            return Err(Error::new("empty query"));
        }
        let res = self.store.search(q).iter().map(|&(_, w)| w).collect::<Vec<&str>>().join("||");
        println!("{}", res);
        Ok(())
    }

    fn list_file(&self, q: &str) -> Result<()> {
        let (dirname, basename) = path_split(q);

        let mut res = Vec::new();
        for entry in try!(dirname.read_dir()) {
            let e = try!(entry);
            let name = e.file_name();
            let (score, is_subseq) = {
                let n = try!(name.to_str().ok_or_else(|| Error::new("invalid filename")));
                utils::is_subseq(basename, n)
            };
            if !basename.is_empty() && !is_subseq {
                continue;
            }
            let sign = {
                let filetype = try!(e.file_type());
                if filetype.is_symlink() {
                    "sym"
                } else if filetype.is_dir() {
                    "dir"
                } else {
                    "file"
                }
            };
            res.push((score, (name, sign)));
        }
        res.sort_by_key(|&(s, (ref n, _))| (s, n.len()));
        let out = res.into_iter()
            .map(|(_, (n, s))| format!("{},,{}", n.to_str().unwrap(), s))
            .collect::<Vec<String>>()
            .join("||");
        println!("{}", out);
        Ok(())
    }

    fn add_content(map: &mut HashMap<String, time::SystemTime>,
                   store: &mut TokenStore,
                   ty: AddType,
                   data: &str)
                   -> Result<()> {
        match ty {
            AddType::File => {
                let meta = try!(fs::metadata(data));
                let modified = try!(meta.modified());
                if map.get(data).map_or(true, |&t| t < modified) {
                    map.insert(data.to_string(), modified);
                    try!(store.add_file(data));
                }
            }
            AddType::Text => store.add_text(data),
        }
        Ok(())
    }
}

fn path_split(p: &str) -> (&Path, &str) {
    let path = Path::new(p);
    if p.ends_with(path::MAIN_SEPARATOR) {
        (path, "")
    } else {
        (path.parent().unwrap_or(path),
         path.file_name()
            .iter()
            .flat_map(|e| e.to_str())
            .next()
            .unwrap_or(""))
    }
}

fn main() {
    let mut manager = Manager::new();
    loop {
        if let Err(e) = manager.process() {
            println!("Err: {}", e);
        }
    }
}

#[test]
fn test_path_split() {
    assert_eq!(path_split("/"), (Path::new("/"), ""));
    assert_eq!(path_split("/etc"), (Path::new("/"), "etc"));
    assert_eq!(path_split("/etc/"), (Path::new("/etc"), ""));
    assert_eq!(path_split("/etc/a"), (Path::new("/etc"), "a"));
}
