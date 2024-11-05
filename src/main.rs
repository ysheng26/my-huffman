use std::{
    cell::RefCell,
    cmp::Reverse,
    collections::{hash_map::Entry, BinaryHeap, HashMap},
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
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(TreeNode { data, left, right }))
    }
}

/// takes Option<&TreeNode> because it does not take ownership of left or right
/// returns Option<TreeNode> because it is a new thing
fn merge(
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    match (left, right) {
        (None, None) => None,
        (None, Some(right_node)) => Some(right_node.clone()),
        (Some(left_node), None) => Some(left_node.clone()),
        (Some(left_node), Some(right_node)) => {
            let new_node = TreeNode::new(None, Some(left_node.clone()), Some(right_node.clone()));
            Some(new_node)
        }
    }
}

fn build_tree(freq: &HashMap<char, i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut max_heap = BinaryHeap::new();
    for (ch, count) in freq.iter() {
        max_heap.push((Reverse(count), ch));
    }
    println!("max_heap {:?}", max_heap);

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
                let merged_node = merge(Some(left_node), Some(right_node));
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

            let merged_node = merge(prev_node.clone(), Some(curr_node));
            if let Some(merged_node) = merged_node {
                prev_node = Some(merged_node);
            }
        }
    }
    // when we break out of the loop prev_node is the root

    println!("prev_node {:?}", prev_node);

    prev_node
}

fn aux(node: Option<Rc<RefCell<TreeNode>>>, curr: &mut Vec<char>, res: &mut HashMap<char, String>) {
    // base case node is None
    // data is not none then we should push to map
    // if left go left
    // if right go right

    match node {
        None => return,
        Some(node) => {
            println!("visiting {:?}", node.borrow().data);
            if !node.borrow().data.is_none() {
                // curr into string then insert to map
                let xs: String = curr.iter().collect();
                res.insert(node.borrow().data.unwrap(), xs);
                println!("base case hit, res {:?}", res);
                return;
            }

            if !node.borrow().left.is_none() {
                curr.push('0');
                println!("before going in left branch. curr {:?}", curr);
                aux(node.borrow().left.clone(), curr, res);
                curr.pop();
                println!("coming out of left branch. curr {:?}", curr);
            }

            if !node.borrow().right.is_none() {
                curr.push('1');
                println!("before going in right branch. curr {:?}", curr);
                aux(node.borrow().right.clone(), curr, res);
                curr.pop();
                println!("coming out of right branch. curr {:?}", curr);
            }
        }
    }
}

// type HuffmanDict = HashMap<char, String>;
// figure out how to do actual binary
fn tree_to_dict(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> (HashMap<char, String>, HashMap<String, String>) {
    let mut encoding_dict = HashMap::new();

    let mut curr = Vec::<char>::new();
    aux(root, &mut curr, &mut encoding_dict);

    // while traversing the tree, left 0 right 1 and keep the path till we get
    // to a leaf node
    //
    // or actually a node with Some data

    let mut decoding_dict = HashMap::<String, String>::new();
    for (ch, s) in encoding_dict.iter() {
        decoding_dict.insert(s.to_string(), ch.to_string());
    }

    (encoding_dict, decoding_dict)
}

/// encodes input string with huffman encoding. Returning encoded string and
/// the decoding dictionary
///
/// TODO: maybe create a type for the dictionary?
fn encode(s: &str) -> (String, HashMap<String, String>) {
    let mut freq = HashMap::new();
    for ch in s.chars() {
        freq.entry(ch).and_modify(|v| *v += 1).or_insert(1);
    }

    let root = build_tree(&freq);
    let (encoding_dict, decoding_dict) = tree_to_dict(root);

    println!("encoding_dict {:?}", encoding_dict);
    println!("decoding_dict {:?}", decoding_dict);

    let encoded_string = s
        .to_string()
        .chars()
        .map(|ch| encoding_dict.get(&ch).unwrap().clone())
        .collect();

    (encoded_string, decoding_dict)
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
