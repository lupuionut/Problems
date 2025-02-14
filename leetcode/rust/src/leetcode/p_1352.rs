// 1352. Product of the Last K Numbers
// -----------------------------------

struct ProductOfNumbers {
    curr: i32,
    prefix: Vec<i32>,
}

impl ProductOfNumbers {
    fn new() -> Self {
        ProductOfNumbers {
            curr: 1,
            prefix: vec![],
        }
    }

    fn add(&mut self, num: i32) {
        if num == 0 {
            self.curr = 1;
            self.prefix = vec![];
        } else {
            self.curr *= num;
            let n = self.prefix.len();
            if n == 0 {
                self.prefix.push(num);
            } else {
                self.prefix.push(self.prefix[n - 1] * num);
            }
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        let n = self.prefix.len();
        if n == 0 {
            return 0;
        }
        if k as usize > n {
            return 0;
        }
        if k as usize == n {
            return self.curr;
        }
        self.curr / self.prefix[n - 1 - k as usize]
    }
}
