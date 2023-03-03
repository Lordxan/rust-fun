use queue::Queue;
use tree::Node;
mod queue;
mod tree;

fn main() {
    let mut graph = Node::new(3);
    graph.create_graph(vec![1, 2, 3, 4, 5]);
    println!("{graph:#?}");

    let mut queue: Queue<i32> = Queue::default();
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
    queue.enqueue(4);
    println!("{:?}", queue.dequeue());
    queue.print();
}
