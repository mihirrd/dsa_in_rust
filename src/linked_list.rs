use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(val: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { val, next: None }))
    }

    fn new2(val: i32, node: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            val,
            next: Some(node),
        }))
    }
}

#[derive(Debug)]
struct LinkedList {
    head: Rc<RefCell<Node>>,
    size: i32,
}

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList {
            head: Node::new(0),
            size: 0,
        }
    }

    fn append(&mut self, val: i32) {
        let new_node = Node::new(val);
        let mut current = Rc::clone(&self.head);

        while let Some(next) = current.clone().borrow().next.clone() {
            current = Rc::clone(&next);
        }

        current.borrow_mut().next = Some(new_node);
        self.size += 1;
    }

    fn pop(&mut self) {
        let mut size = self.size;
        let mut current = Some(Rc::clone(&self.head));

        while size > 1 {
            current = current.unwrap().clone().borrow().next.clone();
            size -= 1;
        }
        current.unwrap().clone().borrow_mut().next = None;
        self.size -= 1;
    }

    fn print_list(&self) {
        let mut current = self.head.borrow().next.clone();

        while let Some(next) = current {
            print!("{:?} ", next.borrow().val);
            current = next.borrow().next.clone();
        }
    }

    fn insert_at(&mut self, index: i32, val: i32) {
        match index {
            0 => {
                let second_node = Rc::clone(&self.head).borrow().next.clone();
                let new_node = Node::new2(val, second_node.unwrap());
                self.head.borrow_mut().next = Some(new_node);
            }

            index if index == self.size => {
                self.append(val);
            }

            index if index > self.size => {
                panic!("List Index out of bounds");
            }

            _ => {
                let mut ptr = 0;
                let mut current = Rc::clone(&self.head);
                while ptr < index {
                    current = current.clone().borrow().next.clone().unwrap();
                    ptr += 1;
                }

                let new_node = Node::new2(val, current.borrow().next.clone().unwrap());
                current.borrow_mut().next = Some(new_node);
            }
        }
        self.size += 1;
    }

    fn remove_at(&mut self, index: i32) {
        if index >= self.size {
            panic!("Index out of bounds");
        }
        match index {
            0 => {
                let first_node = Rc::clone(&self.head).borrow().next.clone().unwrap();
                let next_node = first_node.borrow_mut().next.clone();
                self.head.borrow_mut().next = next_node;
            }

            index if index == self.size - 1 => {
                self.pop();
            }

            _ => {
                let mut ptr = 0;
                let mut current = Rc::clone(&self.head);
                while ptr < index {
                    current = current.clone().borrow().next.clone().unwrap();
                    ptr += 1;
                }

                let next_node = current.borrow().next.clone();
                let next_next = next_node.unwrap().borrow().next.clone();
                current.borrow_mut().next = next_next;
            }
        }
        self.size -= 1;
    }

    fn contains(&self, val: i32) -> bool {
        let mut current = Rc::clone(&self.head);
        while let Some(next) = current.clone().borrow().next.clone() {
            if next.borrow().val == val {
                return true;
            }
            current = Rc::clone(&next);
        }
        return false;
    }
}
