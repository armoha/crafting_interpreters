use std::marker::PhantomData;
use std::mem;
use std::ptr::NonNull;

struct List {
    head: Link,
    tail: Link,
    len: usize,
}

type Link = Option<NonNull<Node>>;

struct Node {
    prev: Link,
    next: Link,
    data: String,
}

impl Node {
    fn new(data: String) -> Self {
        Node {
            next: None,
            prev: None,
            data,
        }
    }

    fn into_inner(self: Box<Self>) -> String {
        self.data
    }
}

impl List {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
            len: 0,
        }
    }

    fn prepend(&mut self, mut node: Box<Node>) {
        // This method takes care not to create mutable references to whole nodes,
        // to maintain validity of aliasing pointers into `element`.
        unsafe {
            node.prev = None;
            node.next = self.head;
            let node = Some(Box::leak(node).into());

            match self.head {
                None => self.tail = node,
                // Not creating new mutable (unique!) references overlapping `element`.
                Some(head) => (*head.as_ptr()).prev = node,
            }

            self.head = node;
            self.len += 1;
        }
    }

    fn append(&mut self, mut node: Box<Node>) {
        // This method takes care not to create mutable references to whole nodes,
        // to maintain validity of aliasing pointers into `element`.
        unsafe {
            node.prev = self.tail;
            node.next = None;
            let node = Some(Box::leak(node).into());

            match self.tail {
                None => self.head = node,
                // Not creating new mutable (unique!) references overlapping `element`.
                Some(tail) => (*tail.as_ptr()).next = node,
            }

            self.tail = node;
            self.len += 1;
        }
    }

    /// Unlinks the specified node from the current list.
    ///
    /// Warning: this will not check that the provided node belongs to the current list.
    fn remove(&mut self, mut node: NonNull<Node>) {
        let node = unsafe { node.as_mut() }; // this one is ours now, we can create an &mut.

        // Not creating new mutable (unique!) references overlapping `element`.
        match node.prev {
            Some(prev) => unsafe { (*prev.as_ptr()).next = node.next },
            // this node is the head node
            None => self.head = node.next,
        };

        match node.next {
            Some(next) => unsafe { (*next.as_ptr()).prev = node.prev },
            // this node is the tail node
            None => self.tail = node.prev,
        };

        self.len -= 1;
    }

    fn pop_front(&mut self) -> Option<String> {
        match self.head {
            None => None,
            Some(head) => unsafe {
                let v = String::new();
                mem::replace(&mut (*head.as_ptr()).data, v);
                self.remove(head);
                Some(v)
            },
        }
    }

    fn pop_back(&mut self) -> Option<String> {
        match self.tail {
            None => None,
            Some(tail) => unsafe {
                let v = String::new();
                mem::replace(&mut (*tail.as_ptr()).data, v);
                self.remove(tail);
                Some(v)
            },
        }
    }

    fn contain(&self, data: &str) -> bool {
        return false;
    }

    fn print_all(&self) {}

    fn rev_print_all(&self) {}

    fn push_front(&mut self, data: String) {
        self.prepend(Box::new(Node::new(data)));
    }

    fn push_back(&mut self, data: String) {
        self.append(Box::new(Node::new(data)));
    }
}

impl<'a> IntoIterator for &'a List {
    type Item = &'a str;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}

struct Iter<'a> {
    head: Link,
    tail: Link,
    len: usize,
    marker: PhantomData<&'a Node>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node| unsafe {
                // Need an unbound lifetime to get 'a
                let node = &*node.as_ptr();
                self.len -= 1;
                self.head = node.next;
                &node.element
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    #[inline]
    fn last(mut self) -> Option<&'a T> {
        self.next_back()
    }
}

fn main() {
    let mut list = List::new();

    list.push_front("test".to_owned());
    list.push_front("exercise".to_owned());
    list.push_back("list".to_owned());
    list.print_all();

    match list.pop_front() {
        Some(s) => print!("pop_front: {}\t", s),
        None => print!("pop_front fail\t"),
    };
    list.print_all();

    println!(r#"contain "test": {}"#, list.contain("test"));
    match list.pop_back() {
        Some(s) => print!("pop_back: {}\t", s),
        None => print!("pop_back fail\t"),
    };
    list.print_all();
    println!(r#"contain "list": {}"#, list.contain("list"));
}
