use linked_list::LinkedList;

struct Stack {
    list: LinkedList
}

impl Stack {
    fn new() -> Self {
        Self {list: LinkedList::new()}
    }

    fn push(&mut self, value: i32) {
        self.list.push(value);
    }

    fn pop(&mut self) -> Option<i32> {
        self.list.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate() {
    }
}
