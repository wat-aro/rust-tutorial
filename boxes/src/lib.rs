use std::ops::Deref;
use List::{Cons, Nil};

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

#[derive(Debug, PartialEq)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

pub fn cons<T>(x: T, xs: List<T>) -> List<T> {
    Cons(x, Box::new(xs))
}

impl<T> List<T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter { list: self }
    }
}

pub struct Iter<'a, T> {
    pub list: &'a List<T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.list {
            Nil => None,
            Cons(x, xs) => {
                self.list = &**xs;
                Some(x)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List::{Cons, Nil};
    use super::*;

    #[test]
    fn box_test() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn cons_a_list() {
        let list = cons(1, cons(2, Nil));

        assert_eq!(list, Cons(1, Box::new(Cons(2, Box::new(Nil)))))
    }

    #[test]
    fn list_next() {
        let list = cons(1, cons(2, Nil));
        let mut list_iter = list.iter();

        assert_eq!(list_iter.next(), Some(&1));
        assert_eq!(list_iter.next(), Some(&2));
        assert_eq!(list_iter.next(), None);
        assert_eq!(list_iter.next(), None);
    }
}
