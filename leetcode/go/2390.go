package leetcode

/*
    2390. Removing Stars From a String
    ----------------------------------
    total - keep track of total stars, each time we encounter a star total will
    increase with 2, since we need to move back 2 positions (his position and deleted
    char position).
    When we encounter a non star char, if total is not 0, swap the values.
*/
func removeStars(s string) string {

    total := 0
    sb := []byte(s)

    for i := 0; i < len(sb); i++ {
        if sb[i] != '*' {
            if total == 0 {
                continue
            }
            j := i - total
            sb[j] = sb[i]
        } else {
            total += 2
        }
    }
    end := len(s) - total
    return string(sb[0:end])
}
