use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// aaabbcd -> a:3, b:2, c:1, d:1
/// need to calculate the weight instead of using a count
///
/// tree is generated bottom up
///
/// pull two less frequent entries and combine them together
/// in this example it would be c and d.
///
///  *
/// c d
///
/// then get two more
///
///  *
/// a b
///
/// then combine them
///
///     *
///  *     *
/// a b   c d
///
/// in case there are odd nodes, the last one is it's own node
///
///       *
///    e     *
///       *     *
///      a b   c d
///
///
/// use a min heap to pull the two minimum freq entries
///
/// combin them into a small tree
///
/// if there is a current temp node then combin this and update current node
/// else go to the loop and pull the next two
///

struct TreeNode {
    pub data: char,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(
        data: char,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Self {
        TreeNode { data, left, right }
    }
}

/// encodes input string with huffman encoding. Returning encoded string and
/// the frequency map
fn encode(s: &str) -> (String, HashMap<char, i32>) {
    let mut freq = HashMap::new();
    for ch in s.chars() {
        freq.entry(ch).and_modify(|v| *v += 1).or_insert(1);
    }

    ("".to_owned(), freq)
}

fn decode(s: &str, freq: &HashMap<char, i32>) -> String {
    "".to_owned()
}

fn main() {
    let input = "Hello, world!";
    println!("encoding {}", input);
    let (encoded, freq) = encode(&input);
    println!("encoded = {}, freq = {:?}", encoded, freq);
}
