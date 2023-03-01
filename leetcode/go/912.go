package leetcode

func SortArray(nums []int) []int {
	end := len(nums)/2
	multiplier := 1

	if len(nums) >= 2 {
		multiplier = 2
	}

	if len(nums)%2 != 0 {
		end += 1
	}
	parts := make([][]int, end)
	for i := 0; i < end; i++ {
		parts[i] = SliceTake(nums[i*multiplier:], multiplier)
	}

	for true {
		if len(parts) == 1 {
			break
		}
		temp := SortArrayMerge(parts[0], parts[1])
		parts = append(parts[2:], temp)
	}
	return parts[0]
}

func SortArrayMerge(a, b []int) []int {

	sorted := []int{}

	for true {
		if len(a) == 0 || len(b) == 0 {
			break
		}

		if a[0] < b[0] {
			sorted = append(sorted, a[0])
			a = a[1:]
		} else {
			sorted = append(sorted, b[0])
			b = b[1:]
		}
	}

	if len(a) != 0 {
		sorted = append(sorted, a...)
	}
	if len(b) != 0 {
		sorted = append(sorted, b...)
	}

	return sorted
}

func SliceTake(xs []int, n int) []int {
	if n == 2 {
		if len(xs) >= n {
			if xs[0] < xs[1] {
				return []int{xs[0], xs[1]}
			} else {
				return []int{xs[1], xs[0]}
			}
		}
	}
	return xs
}
