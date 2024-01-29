// 232. Implement Queue using Stacks
// ---------------------------------

struct MyQueue {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        Self {
            left: vec![],
            right: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        self.left.push(x);
    }

    fn pop(&mut self) -> i32 {
        while self.left.len() > 1 {
            self.right.push(self.left.pop().unwrap());
        }
        let head = self.left.pop().unwrap();
        while self.right.len() > 0 {
            self.left.push(self.right.pop().unwrap());
        }
        head
    }

    fn peek(&mut self) -> i32 {
        while self.left.len() > 1 {
            self.right.push(self.left.pop().unwrap());
        }
        let head = self.left[0];
        while self.right.len() > 0 {
            self.left.push(self.right.pop().unwrap());
        }
        head
    }

    fn empty(&self) -> bool {
        if self.left.len() == 0 {
            return true;
        }
        false
    }
}
