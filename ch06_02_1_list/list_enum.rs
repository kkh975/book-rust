enum Node {
    Empty,
    Cons(i64, Box<Node>),
}

use Node::{Empty, Cons};

fn node(v: i64, link: Box<Node>) -> Box<Node> {
    Box::new(Cons(v, link))
}

fn main() {
    let c = node(10, node(20, node(30, Box::new(Empty))));

    let mut ptr: &Box<Node> = &c;
    loop {
        // &ptr: 참조자를 얻음
        // \*ptr: 역참조를 수행함으로 Box<Node> 타입을 얻음
        // \**ptr: Box<T>에서 T를 역참조하므로 Node 타입을 얻음
        // &**ptr: Node에 대한 참조자를 얻음으로 &Node 타입을 얻음
        let cur_node: &Node = &**ptr;
        match cur_node {
            Empty => break,
            Cons(v, link) => {
                println!("{}", v);
                ptr = &link;
            },
        }
    }

    let x_box = Box::new(100);
    let x_val = *x_box;
    println!("{}", x_val);
}