package leetcode

func StrStr(haystack string, needle string) int {
	ln := len(needle)
	lh := len(haystack)
	if lh < ln {
		return -1
	}
	if lh == ln {
		if haystack == needle {
			return 0
		} else {
			return -1
		}
	}
	end := lh - ln

	for i := 0; i <= end; i++ {
		if haystack[i:i+ln] == needle {
			return i
		}
 	}
 	return -1
}
