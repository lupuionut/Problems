package leetcode

/*
    69. Sqrt(x)
    -----------
*/

func mySqrt(x int) int {
    for i := 0; i <= x; i++ {
        if i * i == x {
            return i
        }
        if i * i > x {
            return i - 1
        }
    }
    return 0
}
