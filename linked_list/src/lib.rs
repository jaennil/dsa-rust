use std::mem;

#[derive(Debug)]
pub struct LinkedList {
    head: Option<Box<Node>>,
}

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        Self { head: None }
    }

    fn push(&mut self, value: i32) {
        let new_node = mem::replace(&mut self.head, None);
        self.head = Some(Box::new(Node {
            value,
            next: new_node,
        }));
    }

    fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.value)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate() {
        let mut linked_list = LinkedList::new();
        linked_list.push(10);
        linked_list.push(41);
        let popped = linked_list.pop();
        assert_eq!(popped.unwrap(), 41);
        let popped = linked_list.pop();
        assert_eq!(popped.unwrap(), 10);
    }
}
