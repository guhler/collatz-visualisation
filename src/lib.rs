use std::collections::HashSet;

#[derive(Debug)]
pub struct TreeNode {
    n: u32,
    children: Vec<TreeNode>,
}

pub fn make_tree(max: u32) -> TreeNode {
    let mut root = TreeNode {
        n: 1,
        children: vec![],
    };

    let mut set = HashSet::new();

    make_children(max, &mut root, &mut set);

    for i in 1..max {
        if !set.contains(&i) {
            dbg!("found all until ", i);
            break;
        }
    }

    return root;
}

fn make_children(max: u32, node: &mut TreeNode, seen: &mut HashSet<u32>) {
    if seen.contains(&node.n) {
        return;
    }
    seen.insert(node.n);

    if node.n * 2 < max {
        node.children.push(TreeNode {
            n: node.n * 2,
            children: vec![],
        });
    }

    if node.n > 0 && (node.n - 1) % 3 == 0 && (node.n - 1) / 3 % 2 == 1 {
        node.children.push(TreeNode {
            n: (node.n - 1) / 3,
            children: vec![],
        });
    }

    for n in node.children.iter_mut() {
        make_children(max, n, seen);
    }
}

pub fn tree_to_graphviz(root: TreeNode) -> String {
    let mut s = "".to_string();
    children_to_gr(&root, &mut s);
    s
}

fn children_to_gr(node: &TreeNode, s: &mut String) {
    for n in node.children.iter() {
        s.push_str(&(n.n.to_string() + "->" + &node.n.to_string() + "\n"));
        children_to_gr(n, s);
    }
}
