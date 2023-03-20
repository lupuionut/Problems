package leetcode

import (
    "fmt"
    "math"
)

func Reverse(x int) int {
    min := -1 << 31
    max := (1 << 31) - 1

    sign := 1
    r := 0
    if x < 0 {
        sign = -1
        x = x * sign
    }

    for true {
        s := fmt.Sprint(x)
        p := len(s) - 1
        t := x % 10
        addition := (t * int(math.Pow10(p)))

        // check for overflow
        if sign == 1 {
            if max - addition < r {
                r = 0
                break
            }
        } else {
            if min - (sign*addition) > (sign*r) {
                fmt.Println(addition, r)
                r = 0
                break
            }
        }
        r += addition

        x = x/10
        if x == 0 {
            break
        }
    }

    r = r * sign
    return r
}
