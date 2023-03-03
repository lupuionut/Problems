package leetcode

func StrStr(haystack string, needle string) int {
	if len(haystack) < len(needle) {
		return -1
	}
	if len(haystack) == len(needle) {
		if haystack == needle {
			return 0
		} else {
			return -1
		}
	}
	end := len(haystack) - len(needle)
	size := len(needle)

	for i := 0; i <= end; i++ {
		if haystack[i:i+size] == needle {
			return i
		}
 	}
 	return -1
}
