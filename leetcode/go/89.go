package leetcode

/*
    89. Gray Code
    -------------
    More info on how to generate gray codes:
    https://www.youtube.com/watch?v=xuw8HSEP_eI
*/

func grayCode(n int) []int {
    base := []int{0}

    for i := 0; i < n; i++ {
        s := len(base) - 1
        for j := s; j >= 0; j-- {
            m := (1 << i) | base[j]
            base = append(base, m)
        }
    }

    return base
}
