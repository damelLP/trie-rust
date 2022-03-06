use std::cell::RefCell;
use std::iter::Iterator;
use std::sync::Arc;

type OptionalAlphabetTrieNode = Option<Arc<RefCell<AlphabetTrieNode>>>;
type AlphabetArrayCell = Arc<RefCell<[OptionalAlphabetTrieNode; 26]>>;

#[derive(Clone, Debug)]
pub struct AlphabetTrieNode {
    is_terminal: bool,
    children: Arc<RefCell<[OptionalAlphabetTrieNode; 26]>>,
}

impl AlphabetTrieNode {
    pub fn new() -> AlphabetTrieNode {
        AlphabetTrieNode {
            is_terminal: false,
            children: Default::default(),
        }
    }
}

pub struct AlphabetTrie {
    starts: AlphabetArrayCell,
}

impl AlphabetTrie {
    pub fn new() -> Self {
        AlphabetTrie {
            starts: Default::default(),
        }
    }

    fn to_idx(c: char) -> usize {
        c as usize - 'a' as usize
    }

    pub fn add_word(&self, word: &str) {
        let mut available = self.starts.clone();
        let total_chars = word.len();

        for (c_idx, letter) in word.to_lowercase().chars().enumerate() {
            let idx = Self::to_idx(letter);
            if available.borrow()[idx].is_none() {
                available.borrow_mut()[idx] = Some(Arc::new(RefCell::new(AlphabetTrieNode::new())));
            }

            let mut op_node: OptionalAlphabetTrieNode = None;
            if available.borrow()[idx].is_some() {
                op_node = available.borrow()[idx].clone();
            }

            if let Some(node) = op_node {
                available = node.borrow_mut().children.clone();
                if c_idx == total_chars - 1 {
                    node.borrow_mut().is_terminal = true;
                }
            }
        }
    }

    pub fn contains_word(&self, word: &str) -> bool {
        let mut available = self.starts.clone();
        let mut node: OptionalAlphabetTrieNode = None;

        for letter in word.to_lowercase().chars() {
            let idx = Self::to_idx(letter);
            node = available.borrow()[idx].clone();

            if let Some(n) = node.clone() {
                available = n.borrow().children.clone();
            } else {
                return false;
            }
        }

        if let Some(n) = node.clone() {
            return n.borrow().is_terminal;
        }

        return false;
    }
}

pub fn print_available(optional_tries: &[OptionalAlphabetTrieNode]) {
    for (idx, op) in optional_tries.iter().enumerate() {
        if let Some(_) = op {
            let letter = (('a' as usize + idx) as u8) as char;
            print!("{}", letter);
        } else {
            print!("N");
        }
    }
    println!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_idx_res() {
        let t = AlphabetTrie::new();
        t.add_word("hello");
        t.add_word("hellow");
        t.add_word("helloworld");

        assert!(t.contains_word("hello"), "doesn't contain word: hello");
        assert!(t.contains_word("hellow"), "doesn't contain word: hellow");
        assert!(
            t.contains_word("helloworld"),
            "doesn't contain word: helloworld"
        );

        assert!(!t.contains_word("h"), "contains word: h");
        assert!(
            !t.contains_word("helloworldadsf"),
            "contains word: helloworldadsf"
        );
    }

    #[test]
    pub fn test_ascii_struture() {
        for c in "abcdefghijklmnopqrstuvwxyz".chars() {
            println!("{}: {}", c, c as usize - 'a' as usize)
        }
    }
}

fn main() {
    println!("Hello, world!");
}
