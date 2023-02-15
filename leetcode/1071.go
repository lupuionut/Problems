package leetcode

func GcdOfStrings(str1 string, str2 string) string {
    gcd := ""
    if len(str1) > len(str2) {
        gcd = gcd_traverse(str1, str2)  
    } else {
        gcd = gcd_traverse(str2, str1)
    }
    return gcd
}

func gcd_traverse(first string, second string) string {
    result := ""

    loop:
    for i := 0; i < len(second); i++ {
        
        test := second[0:i+1]
        
        for j := 0; j < len(first); {
            if j + i + 1 > len(first) {
                continue loop       
            }
            if first[j:j+i+1] != test {
                continue loop
            }
            j += i + 1
        }

        for j := 0; j < len(second); {
            size := len(test)
            if j + size > len(second) {
                continue loop
            }

            if second[j:j+size] != test {
                continue loop
            }
            j += size
        }

        result = test
    }
    
    return result
}