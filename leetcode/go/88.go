package leetcode

/*
    88. Merge Sorted Array
    ----------------------
    Compare each element from first array with the head of the
    second array. Whenever the second array head is smaller, take
    it and swap it with the value from the first array. Insert
    the value taken from first into the second.
    When we finish iterating over the first array, just add the values
    from the second since the second is also sorted.
*/

func merge(nums1 []int, m int, nums2 []int, n int)  {

    if n == 0 {
        return
    }

    insertInSecond := func(x int, nums []int) []int {
        for i := 0; i < len(nums); i++ {
            if nums[i] > x {
                t := nums[i]
                nums[i] = x
                x = t
            }
        }
        nums = append(nums, x)
        return nums
    }

    for i := 0; i < m + n; i++ {
        if i >= m {
            d := i - m
            nums1[i] = nums2[d]
        } else {
            if nums1[i] > nums2[0] {
                t := nums1[i]
                nums1[i] = nums2[0]
                nums2 = insertInSecond(t, nums2[1:])
            }
        }
    }
}
