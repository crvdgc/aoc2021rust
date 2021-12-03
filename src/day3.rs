pub fn part1(input: &str, l: usize) -> u32 {
    let mut n : u32 = 0;
    let mut zero: Vec<u32> = Vec::with_capacity(l);
    for _ in 0..l {
        zero.push(0);
    }
    for line in input.lines() {
        n += 1;
        for (i, c) in line.chars().enumerate() {
            if c == '0' {
                zero[i] += 1;
            }
        }
    }
    let mut base: u32 = 1 << (l - 1);
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    let half = n / 2;
    for i in 0..l {
        if zero[i] > half {
            epsilon += base;
        } else if zero[i] < half {
            gamma += base;
        } else {
            gamma += base;
            epsilon += base;
        }
        base /= 2;
    }
    return gamma * epsilon;
}

pub fn part2(input: &str, l: usize) -> u32 {
    let mut tree = build_tree(l);
    for line in input.lines() {
        record_binary(line, l, &mut tree);
    }
    let o2 = traverse_tree(true, l, &tree);
    let co2 = traverse_tree(false, l, &tree);
    return o2 * co2;
}

enum BinaryTree { // binary prefix tree
   Leaf,
   Node(TreeNode),
}

struct TreeNode {
   // how many leaves in subtree
   left_count: u32,
   left_child: Box<BinaryTree>,
   right_count: u32,
   right_child: Box<BinaryTree>,

}

fn build_tree(l: usize) -> BinaryTree {
    if l == 0 {
        return BinaryTree::Leaf;
    } else {
        let node = TreeNode {
            left_count: 0,
            left_child: Box::new(build_tree(l - 1)),
            right_count: 0,
            right_child: Box::new(build_tree(l - 1)),
        };
        return BinaryTree::Node(node);
    }
}

fn record_binary(s: &str, l: usize, tree: &mut BinaryTree) {
    if l == 0 {
        return;
    } else if let BinaryTree::Node(node) = tree {
        let c = s.chars().nth(0).unwrap();
        if c == '0' {
            node.left_count += 1;
            record_binary(&s[1..], l - 1, &mut node.left_child);
        } else if c == '1' {
            node.right_count += 1;
            record_binary(&s[1..], l - 1, &mut node.right_child);
        } else {
            panic!("Unknown number");
        }
   } else {
       panic!("Tree too short");
   }
}

fn traverse_tree(find_largest: bool, l: usize, tree: &BinaryTree) -> u32 {
    return traverse_tree_go(find_largest, 1 << (l-1), 0, tree);
}

fn traverse_tree_go(find_largest: bool, base: u32, acc: u32, tree: &BinaryTree) -> u32 {
    match tree {
        BinaryTree::Leaf => {
            println!("");
            return acc;
        }
        BinaryTree::Node(node) => {
            if node.left_count == 1 && node.right_count == 0 {
                print!("0");
                return traverse_tree_go(find_largest, base / 2,
                    acc, &node.left_child);
            } else if node.left_count == 0 && node.right_count == 1 {
                print!("1");
                return traverse_tree_go(find_largest, base / 2,
                    acc + base, &node.right_child);
            }
            if find_largest {
                if node.right_count >= node.left_count {
                    print!("1");
                    return traverse_tree_go(find_largest, base / 2,
                        acc + base, &node.right_child);
                } else {
                    print!("0");
                    return traverse_tree_go(find_largest, base / 2,
                        acc, &node.left_child);
                }
            }  else { // find smallest
                if node.left_count <= node.right_count {
                    print!("0");
                    return traverse_tree_go(find_largest, base / 2,
                        acc, &node.left_child);
                } else {
                    print!("1");
                    return traverse_tree_go(find_largest, base / 2,
                        acc + base, &node.right_child);
                }
            }
        }
    }
}
