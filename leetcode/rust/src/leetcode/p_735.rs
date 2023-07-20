// 735. Asteroid Collision
// -----------------------

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = vec![];

        asteroids.iter().for_each(|&a| {
            let last = stack.pop();
            if last.is_some() {
                let last = last.unwrap();
                if last > 0 && a < 0 {
                    if last.abs() > a.abs() {
                        stack.push(last);
                    } else if last.abs() < a.abs() {
                        loop {
                            if stack.len() == 0 {
                                stack.push(a);
                                break;
                            }

                            let l = stack.pop().unwrap();
                            if l != a && l.abs() == a.abs() {
                                break;
                            }
                            if l < 0 {
                                stack.push(l);
                                stack.push(a);
                                break;
                            }
                            if l.abs() > a.abs() {
                                stack.push(l);
                                break;
                            }
                        }
                    }
                } else {
                    stack.push(last);
                    stack.push(a);
                }
            } else {
                stack.push(a);
            }
        });

        stack
    }
}
