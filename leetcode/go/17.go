package leetcode

// 17. Letter Combinations of a Phone Number

func letterCombinations(digits string) []string {

    var answer []string

    hm := make(map[byte][]string)
    hm['2'] = []string{"a", "b", "c"}
    hm['3'] = []string{"d", "e", "f"}
    hm['4'] = []string{"g", "h", "i"}
    hm['5'] = []string{"j", "k", "l"}
    hm['6'] = []string{"m", "n", "o"}
    hm['7'] = []string{"p", "q", "r", "s"}
    hm['8'] = []string{"t", "u", "v"}
    hm['9'] = []string{"w", "x", "y", "z"}

    for {
        if len(digits) == 0 {
            break
        }

        k := digits[0]
        if len(answer) == 0 {
            answer = append(answer, hm[k]...)
        } else {
            temp := []string{}
            for c := range hm[k] {
                for a := range answer {
                    temp = append(temp, answer[a] + hm[k][c])
                }
            }
            answer = temp
        }
        digits = digits[1:]
    }

    return answer
}
