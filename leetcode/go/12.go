package leetcode

func intToRoman(num int) string {
    hm := map[int]string{1:"I", 4: "IV", 5: "V", 9: "IX", 10: "X", 40: "XL", 50: "L", 90: "XC", 100: "C", 400: "CD", 500: "D", 900: "CM", 1000: "M"}

    roman := ""

    // translate from number to roman
    translate := func(n int) (string, int) {
        max := 0
        for k := range hm {
            if k > max && k <= n {
                max = k
            }
        }
        if max == 0 {
            return "", 0
        } else {
            return hm[max], n - max
        }
    }

    // iterate over string, from right to left, and each for iteration multiply by 10
    i := 1
    for {
        n := num%10
        nm := n * i
        var chars string

        // first check if the value we need is already represented in hm
        // if not, translate it
        if value, ok := hm[nm]; ok {
            chars = value
        } else {
            // translate it until the remaning number is 0
            // for ex, from 67 to LXVII
            // first translation since the highest numer is 50 -> L and remains 17
            // second translation is 10 -> LX -> continue with 7
            // next 5 -> LXV -> continue with 2
            // next 1 -> LXVI -> continue with 1
            // next 1 -> LXVII -> number is 0 -> stop
            for {
                char, remains := translate(nm)
                chars += char
                if remains == 0 {
                    break
                }
                nm = remains
            }
        }
        roman = chars + roman

        num = num/10
        i = i*10
        if num == 0 {
            break
        }
    }

    return roman
}
