package leetcode

func isPalindrome(x int) bool {
    if x < 0 {
        return false
    }
    if x == 0 {
        return true
    }
    digits := []int{}

    for true {
        digits = append(digits, x%10)
        x = x/10
        if x == 0 {
            break
        }
    }

    k := len(digits)
    for i := 0; i < k/2; i++ {
        if digits[i] != digits[k-i-1] {
            return false
        }
    }
    return true
}
