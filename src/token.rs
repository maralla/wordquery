use std::collections::BTreeSet;
use std::fs::File;
use std::io::Read;

use errors::Result;
use regex::Regex;
use utils;

lazy_static! {
    static ref WORD: Regex = Regex::new(r"[^\W\d]{3,}\w*").unwrap();
}

pub struct TokenStore {
    db: BTreeSet<String>,
}

impl TokenStore {
    pub fn new() -> TokenStore {
        TokenStore { db: BTreeSet::new() }
    }

    pub fn add_file(&mut self, name: &str) -> Result<()> {
        let mut buf = String::new();
        try!(try!(File::open(name)).read_to_string(&mut buf));
        self.add_text(&buf);
        Ok(())
    }

    pub fn add_text(&mut self, text: &str) {
        for word in WORD.find_iter(text).map(|(s, e)| &text[s..e]) {
            self.db.insert(word.to_string());
        }
    }

    pub fn search(&self, src: &str) -> Vec<(i32, &str)> {
        let mut res = self.db
            .iter()
            .map(|e| (utils::is_subseq(src, e), e as &str))
            .filter(|&((_, s), _)| s)
            .map(|((s, _), e)| (s, e))
            .collect::<Vec<(i32, &str)>>();
        res.sort_by_key(|e| (e.0, e.1.len()));
        res
    }
}

#[test]
fn test_search() {
    let mut store = TokenStore::new();
    store.add_file("tests/test.txt").unwrap();
    assert_eq!(store.search("oc"),
               [(-997, "odchodzi"),
                (-7, "amochodowego"),
                (5, "Gorycz"),
                (6, "mogąc"),
                (8, "społeczeństwa")])
}
