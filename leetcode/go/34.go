package leetcode

/*
    34. Find First and Last Position of Element in Sorted Array
    -----------------------------------------------------------
*/

func SearchRange(nums []int, target int) []int {

    left := -1
    right := -1

    var binSearchTarget func ([]int,int,int,int) int
    var binSearchLeft func(xs []int, x int, start int, end int) int
    var binSearchRight func(xs []int, x int, start int, end int) int

    binSearchTarget = func(xs []int, x, start, end int) int {
        if start > end {
            return -1
        }
        middle := (start + end)/2
        if xs[middle] == x {
            return middle
        }
        if xs[middle] > x {
            return binSearchTarget(xs, x, 0, middle - 1)
        } else {
            return binSearchTarget(xs, x, middle + 1, end)
        }
    }

    binSearchLeft = func(xs []int, x, start, end int) int {
        if start == end {
            if nums[start] != x {
                return start + 1
            }
            return start
        }
        middle := (start + end)/2
        if xs[middle] == x {
            return binSearchLeft(xs, x, 0, middle)
        }
        return binSearchLeft(xs, x, middle + 1, end)
    }

    binSearchRight = func(xs []int, x, start, end int) int {
        if start == end {
            if nums[start] != x {
                return start - 1
            }
            return start
        }
        middle := (start + end)/2
        if xs[middle] == x {
            return binSearchRight(xs, x, middle + 1, len(xs) - 1)
        }
        return binSearchRight(xs, x, start, middle)
    }

    found := binSearchTarget(nums, target, 0, len(nums) - 1)

    if found != -1 {
        if found != 0 && found != len(nums) - 1 {
            if nums[found-1] != nums[found] && nums[found+1] != nums[found] {
                return []int{found, found}
            } else {
                // if previous number is different, this is the start, search for end
                if nums[found-1] != nums[found] {
                    left = found
                    right = binSearchRight(nums, target, found, len(nums) - 1)
                }
                // if next number is different, this is the end, search for start
                if nums[found+1] != nums[found] {
                    right = found
                    left = binSearchLeft(nums, target, 0, found - 1)
                }
                // if in the middle
                if nums[found-1] == nums[found] && nums[found+1] == nums[found] {
                    right = binSearchRight(nums, target, found, len(nums) - 1)
                    left = binSearchLeft(nums, target, 0, found - 1)
                }
            }
        }

        // this is the start, search for end
        if found == 0 {
            left = 0
            right = binSearchRight(nums, target, found, len(nums) - 1)
        }
        // this is the end, search for start
        if found != 0 && found == len(nums) - 1 {
            right = found
            left = binSearchLeft(nums, target, 0, found)
        }
    }

    return []int{left, right}
}
