package leetcode

/*
    258. Add Digits
    ---------------
    If the number is not 0, and the remainder from division with 9
    is 0, the digits sum is 9, else is the remainder.
*/

func addDigits(num int) int {
    remainder := num%9
    if num > 0 && remainder == 0 {
        return 9
    }
    return remainder
}
