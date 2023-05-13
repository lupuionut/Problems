package leetcode

/*
    809. Expressive Words
    ---------------------
*/

func expressiveWords(s string, words []string) int {

    stretch := func(s string) [][]int {
        result := [][]int{}
        current := make([]int, 2)
        current[0] = int(s[0])
        current[1] = 1

        for i := 1; i < len(s); i++ {
            if int(s[i]) != current[0] {
                result = append(result, current)
                current = make([]int, 2)
                current[0] = int(s[i])
                current[1] = 1
            } else {
                current[1] += 1
            }
        }
        result = append(result, current)
        return result
    }

    ss := stretch(s)
    ans := 0

    loop:
    for k := range words {
        ws := stretch(words[k])
        if len(ws) != len(ss) {
            continue loop
        }
        for i := 0; i < len(ss); i++ {
            if ss[i][0] != ws[i][0] {
                continue loop
            }
            if ss[i][1] != ws[i][1] && ss[i][1] < 3 {
                continue loop
            }
            if ss[i][1] < ws[i][1] {
                continue loop
            }
        }
        ans += 1
    }

    return ans
}
