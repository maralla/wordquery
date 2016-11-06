use std::collections::BTreeSet;
use std::fs::File;
use std::io::Read;

use errors::Result;
use regex::Regex;

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

    fn test_subseq(src: &str, target: &str) -> (i32, bool) {
        let mut score = 0;
        let mut src_iter = src.chars();
        let mut ch = match src_iter.next() {
            Some(e) => e,
            None => return (0, false),
        };

        for (i, c) in target.char_indices() {
            if c.len_utf8() != ch.len_utf8() {
                continue;
            }

            if c.to_lowercase()
                .zip(ch.to_lowercase())
                .filter(|&(s, t)| s != t)
                .next()
                .is_none() {
                match i {
                    0 => score = -999,
                    _ => score += i as i32,
                }

                match src_iter.next() {
                    Some(c) => ch = c,
                    None => return (score, true),
                }
            }
        }
        (0, false)
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
            .map(|e| (Self::test_subseq(src, e), e as &str))
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
                (5, "Gorycz"),
                (5, "amochodowego"),
                (6, "mogąc"),
                (8, "społeczeństwa")])
}

#[test]
fn test_subseq() {
    assert_eq!(TokenStore::test_subseq("wop", "world"), (0, false));
    assert_eq!(TokenStore::test_subseq("", "world"), (0, false));

    assert_eq!(TokenStore::test_subseq("w", "world"), (-999, true));
    assert_eq!(TokenStore::test_subseq("wld", "world"), (-992, true));
    assert_eq!(TokenStore::test_subseq("d", "world"), (4, true));
    assert_eq!(TokenStore::test_subseq("od", "world"), (5, true));
    assert_eq!(TokenStore::test_subseq("Od", "world"), (5, true));
}
