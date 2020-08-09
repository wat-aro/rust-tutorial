use List::{Cons, Nil};

fn main() {
    let list = List::cons(1, List::cons(2, List::cons(3, Nil)));
    println!("{:?}", list);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn cons(head: i32, tail: List) -> List {
        Cons(head, Box::new(tail))
    }
}
