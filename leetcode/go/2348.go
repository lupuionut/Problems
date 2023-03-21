package leetcode

// traverse the slice and count the length of interval where 0 appears
// based on https://en.wikipedia.org/wiki/Triangular_number we can calculate
// how many times 0 appears for a certain interval
// for ex: 0 0 0 -> n = 3 => subarrays = n * (n+1) / 2 <=> subarrays = 3*4/2 = 6

func zeroFilledSubarray(nums []int) int64 {

    count := 0
    acc := 0

    for k := range nums {
        if nums[k] == 0 {
            acc += 1
        } else {
            if acc != 0 {
                count += acc * (acc + 1) / 2
            }
            acc = 0
        }
    }

    // if the slice ends with a 0 and acc wasn't added to intervals
    if acc != 0 {
        count += acc * (acc + 1) / 2
    }

    return int64(count)
}
