// 341. Flatten Nested List Iterator
// ---------------------------------

struct NestedIterator {
    elements: Vec<i32>,
    position: usize,
    len: usize,
}

impl NestedIterator {
    fn new(nestedList: Vec<NestedInteger>) -> Self {
        fn flat(list: &Vec<NestedInteger>) -> Vec<i32> {
            let mut ans = vec![];

            for i in 0..list.len() {
                match &list[i] {
                    NestedInteger::Int(i) => ans.push(*i),
                    NestedInteger::List(v) => {
                        flat(&v).iter().for_each(|x| {
                            ans.push(*x);
                        });
                    }
                }
            }

            ans
        }

        let elements = flat(&nestedList);
        let len = elements.len();
        Self {
            elements: elements,
            position: 0,
            len: len,
        }
    }

    fn next(&mut self) -> i32 {
        if self.has_next() {
            let value = self.elements[self.position];
            self.position += 1;
            return value;
        }
        -1
    }

    fn has_next(&self) -> bool {
        self.position < self.len
    }
}
