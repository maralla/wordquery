struct Node {
    c: char,
    eow: bool,
    next: Vec<Box<Node>>,
}

impl Node {
    fn new(c: char, eow: bool) -> Node {
        Node {
            c: c,
            eow: eow,
            next: Vec::new(),
        }
    }
}

struct Trie {
    root: Vec<Box<Node>>,
}

impl Trie {
    fn new() -> Trie {
        Trie { root: Vec::new() }
    }

    fn insert(&mut self, word: &str) {
        let mut entry = &self.root;
        for (pos, c) in word.char_indices() {
            match entry.iter().position(|item| item.c == c) {
                Some(i) => {
                    if pos == word.len() - 1 {
                        entry[i].eow = true;
                    }
                    entry = &entry[i].next;
                }
                None => {
                    let node = Node::new(c, pos == word.len() - 1);
                    entry.push(Box::new(node));
                    entry = entry.last().unwrap();
                }
            }
        }
    }

    fn print(&self) -> Vec<String> {
        let mut entry = &self.root;
        word = String::new();
        for node in entry.iter() {
            word.push(node.c);
        }
    }

    fn get_one_word(node: &Node, word: &mut String) {
        word.push(node.c);
        if node.eow {
            return;
        }
    }
}
