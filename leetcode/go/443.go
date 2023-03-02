package leetcode

import (
	"fmt"
)

func Compress(chars []byte) int {
	cursor := 0
	counter := 1
	remains := chars[0]

	// iterate through all until the last one
	for i := 0; i < len(chars) - 1; i++ {
		// if next char is the same as current increase the counter with 1
		if chars[i] == chars[i+1] {
		 	counter += 1
		 	remains = chars[i+1]
		} else {
			chars[cursor] = chars[i]
			if counter > 1 {
				counterBytes := IntToByteArray(counter)
				for j := 0; j < len(counterBytes); j++ {
					cursor += 1
					chars[cursor] = counterBytes[j]
				}
			}
			cursor += 1
			counter = 1
			remains = chars[i+1]
		}
	}

	// if the counter is not reseted to 1, the last byte will be equal with the
	// one before
	if counter != 1 {
		chars[cursor] = remains
		counter := IntToByteArray(counter)
		cursor += 1
		for j := 0; j < len(counter); j++ {
			chars[cursor + j] = counter[j]
		}
		cursor += len(counter)
	} else {
		chars[cursor] = remains
		cursor += 1
	}

	return len(chars[0:cursor])
}

func IntToByteArray(n int) []byte {
	s := fmt.Sprint(n)
	a := []byte{}
	for k := range s {
		a = append(a, s[k])
	}
	return a
}
