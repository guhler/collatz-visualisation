use collaz::*;

fn main() {
    let t = make_tree(4096 * 2);

    let s = tree_to_graphviz(t);
    println!("{s}");
}
