package leetcode

func ThreeSum(nums []int) [][]int {
    sums := [][]int{}
    hm := make(map[int]int)

    for k := range nums {
        hm[nums[k]] += 1
    }

    visited := [][]int{}

    sort := func(xs []int) []int {
        for i := 0; i < len(xs)-1; i++ {
            for j := 0; j < len(xs)-1; j++ {
                if xs[j+1] < xs[j] {
                    temp := xs[j]
                    xs[j] = xs[j+1]
                    xs[j+1] = temp
                }
            }
        }
        return xs
    }

    for k := range hm {
        for i := range hm {
            search := 0 - k - i
            if hm[search] > 0 {
                if (i == k && hm[i] < 2) {
                    continue
                }
                if (search == i || search == k) && hm[search] < 2 {
                    continue
                }

                if (search == i && search == k) && hm[search] < 3 {
                    continue
                }

                n := []int{k, i, search}
                n = sort(n)

                if !isVisited(visited, n) {
                    sums = append(sums, n)
                    visited = append(visited, n)
                }
            }
        }
    }

    return sums
}

func isVisited(xs [][]int, x []int) bool {
    for k := range xs {
        if xs[k][0] == x[0] && xs[k][1] == x[1] && xs[k][2] == x[2] {
            return true
        }
    }
    return false
}
