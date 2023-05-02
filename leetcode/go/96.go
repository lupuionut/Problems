package leetcode
/*
    96. Unique Binary Search Trees
    ------------------------------
    Catalan numbers
*/

func numTrees(n int) int {
    bst := make([]int, n+1)
    bst[0] = 1
    bst[1] = 1

    for nodes := 2; nodes < n + 1; nodes++ {
        total := 0
        for root := 1; root < nodes + 1; root++ {
            left := root - 1
            right := nodes - root
            total += bst[left] * bst[right]
        }
        bst[nodes] = total
    }

    return bst[n]
}
