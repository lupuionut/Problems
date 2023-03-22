package leetcode

func romanToInt(s string) int {
    hm := map[byte]int{'I': 1, 'V': 5, 'X': 10, 'L': 50, 'C': 100, 'D': 500, 'M': 1000}
    ex := map[string]int{"IV": 4, "IX": 9, "XL": 40, "XC": 90, "CD": 400, "CM": 900}
    number := 0
    for {
        if len(s) == 0 {
            break
        }
        if len(s) == 1 {
            number += hm[s[0]]
            s = s[1:]
        } else {
            p := string(s[0]) + string(s[1])
            if v, ok := ex[p]; ok {
                number += v
                s = s[2:]
            } else {
                number += hm[s[0]]
                s = s[1:]
            }
        }
    }

    return number
}
