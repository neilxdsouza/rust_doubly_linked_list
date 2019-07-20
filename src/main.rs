#![feature(core_intrinsics)]

use std::mem;
use std::ptr;

pub struct Node {
    data: i32 ,
}

pub struct Link {
    next: Option<Box<Link>>,
    //prev: Option<Box<Node>>,
    prev: *mut Link,
    node : Node,
}

pub struct List {
    head: Option<Box<Link>>,
    tail: *mut Link,
}

impl Node {
    pub fn new(data: i32) -> Self {
        Self { data }
    }
}

//impl Link {
//    pub fn new(node: Node) -> Self {
//        Self { node, next: None, prev: ptr::null_mut() }
//    }
//}

fn print_type_of<T>(_: &T) {
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}

impl List {
    pub fn new() -> Self {
        Self { head: None, tail: ptr::null_mut() }
    }

    pub fn add_front(&mut self, data: i32) {
        //let new_head: Li
        let n = Node::new(data);

        match self.head {
            None => {
                let mut l = Box::new(Link {
                    node:n,
                    next: None,
                    prev: ptr::null_mut()
                });
                let raw_tail: *mut _ = &mut * l;
                self.head = Some(l);
                self.tail = raw_tail;
            }
            Some (ref mut boxed_link) => {
                print_type_of(boxed_link);
                //let raw_prev_head: * mut Link = Box::into_raw(boxed_link);
                let raw_prev_head: * mut Link = &mut ** boxed_link;
                let mut l = Box::new(Link {
                    node:n,
                    next: mem::replace(&mut self.head, None),
                    prev: raw_prev_head
                    //prev: ptr::null_mut()
                });
                self.head = Some(l);
                //l.next = Some(Box::new(boxed_link.node));
                //unsafe {
                //    *boxed_link.prev = l;
                //}
                //self.head = l;
            }
        }

        

        //if self.head.is_none() {
        //    let n = Node::new(data);
        //    let mut l = Box::new(Link{node:n,
        //        next: None,
        //        prev: ptr::null_mut()
        //    });
        //    let raw_tail: *mut _ = &mut * l;
        //    self.head = Some(l);
        //    self.tail = raw_tail;
        //} else {
        //    let n = Node::new(data);
        //    let raw_prev_head: * mut _ = &mut *self.head;
        //    let mut l = Box::new(Link{node:n,
        //        next: mem::replace(&mut self.head, None),
        //        prev: raw_prev_head
        //    });
        //    self.head = Some(l);
        //    match self.head {
        //        Some (ref mut h2) => {
        //            match (h2) => {
        //                Some(ref mut l2) => {
        //                    //unsafe {
        //                    //    *l2.prev = *l; 
        //                    //}
        //                    l2.prev = Box::into_raw( l); 
        //                    //unsafe {
        //                    //}
        //                },
        //                None => {}
        //            }
        //        },
        //        None => { }
        //    }
        //    //let raw_prev: *mut _ = &mut 
        //    //match self.head {
        //    //    Some(ref mut h) => {
        //    //        //l.prev = Box::into_raw(h);
        //    //        //l.prev = Box::into_raw(h);
        //    //        unsafe {
        //    //        *((l.next).prev) = &mut * l;
        //    //        }
        //    //    }
        //    //    None => {
        //    //    }
        //    //}
        //}
    }

    fn print() {

    }

}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        list.add_front(1);
        list.add_front(2);
        list.add_front(3);
        assert_eq!(2,2);
    }

}

/*
enum Link {
    Empty,
    Has(Box<Node>),
}

impl Node {
    pub fn new(data: i32) -> Self {
        Node { data: data, next: Link::Empty, prev: Link::Empty }
    }
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty, tail: Link::Empty }
    }

    pub fn push(&mut self, data: i32) {
        let new_node: Box<Node> = Box::new(Node::new(data));
        let mut current_head: Link = mem::replace(&mut self.head, Link::Empty);
        match current_head {
            Link::Empty => { self.head = Link::Has(new_node); }
            Link::Has(ref mut a_node) => {
                //a_node.next = self.head.next;
                //self.head = Link::Has(a_node);
                //match a_node {
                //    Link::Empty => {  }
                //    Link::Has(ref mut b_node) => { }
                //}
                a_node.
            }
        }
    }

}
*/


fn main() {
    println!("Hello, world!");
        let mut list = List::new();
        list.add_front(1);
        list.add_front(2);
}
