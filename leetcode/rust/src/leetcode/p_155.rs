// 155. Min Stack
// --------------
// Use a vec of (val, curr_min_key)
// Each time we add a value to the stack compare the value
// with the one corresponding to the curr_min_key
// if it's smaller, push the new element with a curr_min_key
// corresponding to its own index.

struct MinStack {
    stack: Vec<(i32, usize)>,
    cursor: usize,
}

impl MinStack {
    fn new() -> Self {
        let stack: Vec<(i32, usize)> = Vec::new();
        let cursor = 0;
        Self { stack, cursor }
    }

    fn push(&mut self, val: i32) {
        if self.stack.len() == 0 {
            self.stack.push((val, self.cursor));
        } else {
            let last = self.stack[self.cursor - 1];
            let mut curr_min_key = last.1;
            if self.stack[curr_min_key].0 > val {
                curr_min_key = self.cursor;
            }
            self.stack.push((val, curr_min_key));
        }
        self.cursor += 1;
    }

    fn pop(&mut self) {
        self.stack.pop();
        self.cursor -= 1;
    }

    fn top(&self) -> i32 {
        return self.stack[self.cursor - 1].0;
    }

    fn get_min(&self) -> i32 {
        let curr_min_key = self.stack[self.cursor - 1].1;
        self.stack[curr_min_key].0
    }
}
