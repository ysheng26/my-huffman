use std::{
    cell::RefCell,
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    rc::Rc,
};

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

// type _TreeNode = Option<Rc<RefCell<TreeNode>>>;

#[derive(Clone, Debug)]
struct TreeNode {
    pub data: Option<char>,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(
        data: Option<char>,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Self {
        TreeNode { data, left, right }
    }
}

/// takes Option<&TreeNode> because it does not take ownership of left or right
/// returns Option<TreeNode> because it is a new thing
fn merge(left: Option<&TreeNode>, right: Option<&TreeNode>) -> Option<TreeNode> {
    match (left, right) {
        (None, None) => None,
        (None, Some(right_node)) => Some(right_node.clone()),
        (Some(left_node), None) => Some(left_node.clone()),
        (Some(left_node), Some(right_node)) => {
            let new_node = TreeNode::new(
                None,
                Some(Rc::new(RefCell::new(left_node.clone()))),
                Some(Rc::new(RefCell::new(right_node.clone()))),
            );
            Some(new_node)
        }
    }
}

/// encodes input string with huffman encoding. Returning encoded string and
/// the decoding dictionary
///
/// TODO: maybe create a type for the dictionary?
fn encode(s: &str) -> (String, HashMap<char, i32>) {
    // fn encode(s: &str) -> (String, HashMap<char, String>) {
    let mut freq = HashMap::new();
    for ch in s.chars() {
        freq.entry(ch).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut max_heap = BinaryHeap::new();
    for (ch, count) in freq.iter() {
        max_heap.push((Reverse(count), ch));
    }

    // build tree with max_heap
    // [1, 1, 2, 3, 4]

    // let mut prev_node = TreeNode::new(None, None, None);

    let mut prev_node = None;
    while !max_heap.is_empty() {
        let a = max_heap.pop();
        let b = max_heap.pop();

        let mut curr_node = TreeNode::new(None, None, None);
        match (a, b) {
            (None, None) => {
                // might have an edge case here
                break;
            }
            (None, Some((_, ch))) => curr_node = TreeNode::new(Some(*ch), None, None),
            (Some((_, ch)), None) => {
                curr_node = TreeNode::new(Some(*ch), None, None);
            }
            (Some((_, left_ch)), Some((_, right_ch))) => {
                let left_node = TreeNode::new(Some(*left_ch), None, None);
                let right_node = TreeNode::new(Some(*right_ch), None, None);
                let merged_node = merge(Some(&left_node), Some(&right_node));
                if let Some(merged_node) = merged_node {
                    curr_node = merged_node;
                }
            }
        }

        if prev_node.is_none() {
            prev_node = Some(curr_node);
        } else {
            // there was some node wating here.
            // merge prev_node and curr_node and set prev_node to the newly
            // merged node

            let merged_node = merge(prev_node.as_ref(), Some(&curr_node));
            if let Some(merged_node) = merged_node {
                prev_node = Some(merged_node);
            }
        }
    }
    // when we break out of the loop prev_node is the root

    println!("max_heap {:?}", max_heap);
    println!("prev_node {:?}", prev_node.unwrap());

    ("".to_owned(), freq)
}

fn decode(s: &str, freq: &HashMap<char, i32>) -> String {
    "".to_owned()
}

fn main() {
    let input = "abaabcde";
    println!("encoding {}", input);
    let (encoded, freq) = encode(&input);
    println!("encoded = {}, freq = {:?}", encoded, freq);
}
