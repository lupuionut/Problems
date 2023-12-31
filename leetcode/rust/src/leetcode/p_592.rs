// 592. Fraction Addition and Subtraction
// --------------------------------------

#[derive(PartialEq, Debug, Clone)]
enum Token {
    Numarator,
    Numitor,
    Sign,
    Divisor,
    None,
}

impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        let mut numaratori = vec![];
        let mut numitori = vec![];
        let mut signs = vec![];
        let mut last = Token::None;

        expression.chars().for_each(|c| match c {
            '-' => {
                signs.push(-1);
                last = Token::Sign;
            }
            '+' => {
                signs.push(1);
                last = Token::Sign;
            }
            '/' => {
                last = Token::Divisor;
            }
            _ => {
                if last == Token::None {
                    let value = c.to_digit(10).unwrap() as i32;
                    numaratori.push(value);
                    signs.push(1);
                    last = Token::Numarator;
                } else if last == Token::Sign {
                    let value = c.to_digit(10).unwrap() as i32;
                    numaratori.push(value);
                    last = Token::Numarator;
                } else if last == Token::Numarator {
                    let value = numaratori.pop().unwrap();
                    let value = value * 10 + (c.to_digit(10).unwrap() as i32);
                    numaratori.push(value);
                } else if last == Token::Divisor {
                    let value = c.to_digit(10).unwrap() as i32;
                    numitori.push(value);
                    last = Token::Numitor;
                } else if last == Token::Numitor {
                    let value = numitori.pop().unwrap();
                    let value = value * 10 + (c.to_digit(10).unwrap() as i32);
                    numitori.push(value);
                } else {
                    panic!("{:?} something is wrong", (c, last.clone()));
                }
            }
        });

        let mut least = 1;
        numitori.iter().for_each(|&n| {
            least = Solution::lcm(least, n);
        });
        let mut top = 0;
        for i in 0..numitori.len() {
            top += (numaratori[i] * (least / numitori[i]) * signs[i]);
        }
        let mut reductor = Solution::gcd(top, least).abs();
        let top = top / reductor;
        let least = least / reductor;
        let result = format!("{}/{}", top, least);
        result
    }

    fn lcm(a: i32, b: i32) -> i32 {
        (a * b).abs() / Solution::gcd(a, b)
    }

    fn gcd(a: i32, b: i32) -> i32 {
        let mut a = a;
        let mut b = b;
        if b == 0 {
            a
        } else {
            Solution::gcd(b, a % b)
        }
    }
}
