use List::{Cons, Nil};

fn main() {
    let list = List::cons(1, List::cons(2, List::cons(3, Nil)));
    println!("{:?}", list);
    println!("{:?}", List::cdr(&list));
    println!("{:?}", List::cdr(List::cdr(&list)));
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use std::ops::Deref;

impl List {
    fn cons(head: i32, tail: List) -> List {
        Cons(head, Box::new(tail))
    }

    fn cdr(list: &List) -> &List {
        match list {
            Nil => &Nil,
            Cons(_, rest) => rest.deref()
        }
    }
}
