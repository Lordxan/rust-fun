use itertools::Itertools;

#[derive(Debug)]
pub(crate) struct Node {
    pub(crate) right: Option<Box<Node>>,
    pub(crate) left: Option<Box<Node>>,
    pub(crate) value: i32,
}
impl Node {
    pub(crate) fn create_graph(&mut self, arr: Vec<i32>) {
        let sorted = arr.iter().sorted().collect::<Vec<_>>();
        for item in sorted {
            self.insert(Node::new(*item));
        }
    }
    pub(crate) fn insert(&mut self, new_node: Node) {
        let is_left = self.value > new_node.value;
        let is_same = self.value == new_node.value;
        match (&mut self.left, &mut self.right, is_left, is_same) {
            (None, Some(_), true, false) => {
                self.left = Option::Some(Box::new(new_node));
            }
            (None, Some(r), false, false) => {
                Node::insert(r, new_node);
            }
            (Some(l), None, true, false) => {
                Node::insert(l, new_node);
            }
            (Some(_), None, false, false) => {
                self.right = Option::Some(Box::new(new_node));
            }
            (None, None, true, false) => {
                self.left = Option::Some(Box::new(new_node));
            }
            (None, None, false, false) => {
                self.right = Option::Some(Box::new(new_node));
            }
            (Some(l), Some(_), true, false) => {
                Node::insert(l, new_node);
            }
            (Some(_), Some(r), false, false) => {
                Node::insert(r, new_node);
            }
            _ => {}
        }
    }
    pub(crate) fn new(value: i32) -> Node {
        Node {
            value,
            right: None,
            left: None,
        }
    }
}
