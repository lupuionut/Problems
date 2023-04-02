package leetcode

/*
    29. Divide Two Integers
    -----------------------
*/
func Divide(dividend int, divisor int) int {
    min := -2147483648
    max := 2147483647
    quotient := 0

    dv := dividend
    ds := divisor

    if dividend == min && divisor == -1 {
        return max
    }
    if divisor == 1 {
        return dividend
    }
    if divisor == -1 {
        return 0 - dividend
    }

    if dividend < 0 {
        dv = 0 - dividend
    }
    if divisor < 0 {
        ds = 0 - divisor
    }

    for dv >= ds {
        newdivisor := ds
        multiplier := 1
        for dv >= newdivisor {
            quotient += multiplier
            dv -= newdivisor
            multiplier += multiplier
            newdivisor += newdivisor
        }
    }

    if (dividend < 0 && divisor > 0) || (dividend > 0 && divisor < -0) {
        return 0 - quotient
    }

    return quotient
}
