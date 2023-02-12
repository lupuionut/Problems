package leetcode

func DistinctNames(ideas []string) int {
    
    count := 0
    ideasMap := make(map[string]bool)
    letters := map[rune]int{'a': 0, 'b': 1, 'c': 2, 'd': 3, 'e': 4, 'f': 5, 
                        'g': 6, 'h': 7, 'i': 8, 'j': 9, 'k': 10, 'l': 11, 'm': 12,
                        'n': 13, 'o': 14, 'p': 15, 'q': 16, 'r': 17, 's': 18, 't': 19,
                        'u': 20, 'v': 21, 'w': 22, 'y': 23, 'z': 24, 'x': 25}
        
    var groups [26][]string
    for _, v := range(ideas) {
        ideasMap[v] = true        
        key := letters[rune(v[0])]
        groups[key] = append(groups[key], v)
    }

    l, r := 0, 0
    
    for i := 0; i < 25; i++ {
        if len(groups[i]) > 0 {
            for j := i+1; j <= 25; j++ {
                l, r = 0, 0
                if len(groups[j]) > 0 {
                    for _, word := range(groups[i]) {
                        subword := string(groups[j][0][0]) + word[1:]
                        if !ideasMap[subword] {
                            l += 1
                        }
                    }
                    for _, word := range(groups[j]) {
                        subword := string(groups[i][0][0]) + word[1:]
                        if !ideasMap[subword] {
                            r += 1
                        }
                    }
                }
                count += 2 * l * r
            }
        }
    }    
    return count
}
