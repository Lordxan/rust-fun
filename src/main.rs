mod tree;
use tree::Node;

fn main() {
    let mut root = Node {
        value: 3,
        right: None,
        left: None,
    };
    root.create_graph(vec![1, 2, 3, 4, 5]);
    println!("{:#?}", root)
}
