package leetcode

/*
    we should go until half the sum length of the 2 arrays and keep track
    of the last 2 elements

    for each iteration, compare the element at index i1 from first array with
    the element at index i2 from second array

    take the smallest of them, and increase the index of the corresponding array
*/
func FindMedianSortedArrays(nums1 []int, nums2 []int) float64 {
    total := len(nums1) + len(nums2)
    max := total/2
    previous, current := 0, 0
    i1, i2 := 0, 0

    for i := 0; i <= max; i++ {
        previous = current

        // if we finished array1, just continue to take from array2
        if i1 >= len(nums1) {
            current = nums2[i2]
            i2 += 1
            continue
        }

        // if we finished array2, just continue to take from array1
        if i2 >= len(nums2) {
            current = nums1[i1]
            i1 += 1
            continue
        }

        // take the smallest element from array1, array2
        if nums1[i1] < nums2[i2] {
            current = nums1[i1]
            i1 += 1
        } else {
            current = nums2[i2]
            i2 += 1
        }
    }

    // for odd length, we just take the current one who is at half of array1 + array2
    if total & 1 == 1 {
        return float64(current)
    }
    // for even length, we must split the (half-1) + half / 2
    return float64(previous+current)/float64(2)
}
