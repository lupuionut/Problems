package leetcode

func myAtoi(s string) int {
    max := (1 << 31) -1
    min := (-1 << 31)
    sign := 1
    n := 0

    numbers := map[byte]int {48:0, 49: 1, 50:2, 51:3, 52: 4, 53:5, 54: 6, 55: 7, 56: 8, 57: 9}
    plus := byte(43)
    minus := byte(45)
    comma := byte(46)
    init := false

    for k := range s {
        if s[k] == plus || s[k] == minus {
            if init == true {
                break
            } else {
                init = true
                if s[k] == minus {
                    sign = -1
                }
            }
            continue
        }
        if s[k] == comma && n == 0  || s[k] == comma && init == true {
            break
        }
        if s[k] >= 48 && s[k] <= 57 {
            init = true
            if (n * sign) < 0 && (n*sign) < min {
                return min
            }
            if (n * sign) > 0 && n > max {
                return max
            }
            n = (n*10) + numbers[s[k]]
        } else {
            if (init == false && s[k] != 32) ||  init == true {
                break
            }
        }
    }

    n = n * sign

    if n < min {
        n = min
    }
    if n > max {
        n = max
    }
    return n
}
