package leetcode

/*
    43. Multiply Strings
    --------------------
        123 *
         35 =
        615
       369
      -----
       4305

    Just form array for each mutiplication and add them (615 + 3690)
*/

func multiply(num1 string, num2 string) string {

    if num1 == "0" || num2 == "0" {
        return "0"
    }

    results := []string{}
    cti := make(map[byte]int)
    itc := make(map[int]string)
    chars := []byte{48, 49, 50, 51, 52, 53, 54, 55, 56, 57}
    for i := 0; i < 10; i++ {
        cti[chars[i]] = i
        itc[i] = string(chars[i])
    }

    end := ""
    max := 0

    for i := len(num1) - 1; i >= 0; i-- {
        rez := ""
        carry := 0
        for j := len(num2) - 1; j >= 0; j-- {
            r := (cti[num1[i]] * cti[num2[j]]) + carry
            carry = r / 10
            r = r % 10
            rez = itc[r] + rez
        }
        if carry != 0 {
            rez = itc[carry] + rez
        }
        prod := rez + end
        if len(prod) > max {
            max = len(prod)
        }
        results = append(results, prod)
        end += "0"
    }

    carry := 0
    final := ""
    for i := 0; i < max; i++ {
        t := 0
        for k := range results {
            if len(results[k]) == 0 {
                continue
            }
            t += cti[results[k][len(results[k])-1]]
            results[k] = results[k][0 : len(results[k])-1]
        }
        t += carry
        carry = t / 10
        t = t % 10
        final = itc[t] + final
    }

    if carry != 0 {
        final = itc[carry] + final
    }

    return final
}
