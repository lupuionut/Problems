package leetcode

/*
    191. Number of 1 Bits
    ---------------------
    If the num is odd, the last bit will be 1,
    so we add the last bit and we continue by
    dropping the last bit and check the next number.
*/

func hammingWeight(num uint32) int {
    if num == 0 {
        return 0
    }
    return (int(num) & 1) + hammingWeight(num >> 1)
}
