package leetcode

/*
    30. Substring with Concatenation of All Words
    ---------------------------------------------
    Create a map with all words that may compose a substring and their
    frequency.
    Iterate the string from 0 to length of string - length of substring and
    use a window of substring length.
    For ex: string: "abcdefgh" and substring length is 3 => "abc", "bcd", "def", "fgh"
    For each possible substring check if all words from map are present in the
    substring.
*/

func findSubstring(s string, words []string) []int {

    wm := make(map[string]int)
    wordlen := len(words[0])
    sublen := wordlen * len(words)
    occurences := []int{}

    // if string length is smaller then substring length
    if len(s) < sublen {
        return nil
    }

    // create the map with each word frequency
    for k:= range words {
        if val, ok := wm[words[k]]; ok {
            wm[words[k]] = val + 1
        } else {
            wm[words[k]] = 1
        }
    }

    loop:
    for i := 0; i <= len(s) - sublen; i++ {
        substring := s[i:i+sublen]
        visited := make(map[string]int)
        for j := 0; j < sublen; {
            word := substring[j:j+wordlen]
            visited[word] += 1
            if visited[word] > wm[word] {
                continue loop
            }
            j += wordlen
        }
        occurences = append(occurences, i)
    }

    return occurences
}
