package leetcode

/*
    1813. Sentence Similarity III
    -----------------------------
    For each word, trim from left and from right.
    If any of these 2 remains empty, the ans is true.
*/

import "strings"

func areSentencesSimilar(sentence1 string, sentence2 string) bool {
    f := strings.Split(sentence1, " ")
    s := strings.Split(sentence2, " ")

    common := func(a, b []string) bool {
        for len(a) > 0 && len(b) > 0 {
            if a[0] == b[0] {
                a = a[1:]
                b = b[1:]
            } else {
                break
            }
        }

        for len(a) > 0 && len(b) > 0 {
            la := len(a)
            lb := len(b)
            if a[la-1] == b[lb-1] {
                a = a[0:la-1]
                b = b[0:lb-1]
            } else{
                break
            }
        }

        return len(a) == 0
    }

    return common(f, s) || common(s, f)
}
