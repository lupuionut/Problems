package leetcode

func AddToArrayForm(num []int, k int) []int {
    second := numberToArray(k)
    carry := 0
    acc := []int{}
    max := len(num) - 1
    if len(second) > max {
        max = len(second) - 1
    }
    for k := max; k >= 0; k-- {
        a := arrayValue(num, max, k)
        b := arrayValue(second, max, k)
        n := a+b+carry
        if n >= 10 {
            n = n%10
            carry = 1
        } else {
            carry = 0
        }
        acc = append(acc, n)
    } 
    if carry == 1 {
        acc = append(acc, 1)
    }
    return inverse(acc)
}

func numberToArray(num int) []int {
    var acc []int
    var inverse []int
    for num != 0 {
        acc = append(acc, num%10)
        num = num/10
    }
    for k := len(acc) - 1; k >= 0; k-- {
        inverse = append(inverse, acc[k])
    }
    return inverse
}

func arrayValue(xs []int, max int, k int) int {
	l := len(xs) - 1
	if l < max {
		if max-k <= l && max-k >= 0 {
			return xs[l-(max-k)]
		}
		return 0
	}
	return xs[k]
}

func inverse(xs []int) []int {
    inverse := []int{}
    for k := len(xs) - 1; k >= 0; k-- {
        inverse = append(inverse, xs[k])
    }
    return inverse
}