package leetcode

/*
    58. Length of Last Word
    -----------------------
*/

func lengthOfLastWord(s string) int {
    temp, last := []byte{}, []byte{}

    for k := range s {
        if s[k] == 32 {
            if len(temp) > 0 {
                last = temp
                temp = []byte{}
            }
        } else {
            temp = append(temp, s[k])
        }
    }

    if len(temp) > 0 {
        return len(temp)
    }
    return len(last)
}
