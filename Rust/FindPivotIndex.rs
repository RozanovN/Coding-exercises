// Given an array of integers nums, calculate the pivot index of this array.
//
// The pivot index is the index where the sum of all the numbers strictly to the left of the index is equal to the sum of all the numbers strictly to the index's right.
//
// If the index is on the left edge of the array, then the left sum is 0 because there are no elements to the left. This also applies to the right edge of the array.
//
// Return the leftmost pivot index. If no such index exists, return -1.
// https://leetcode.com/problems/find-pivot-index/

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut index = 0;
        while index < nums.len()
        {
            let left_slice = &nums[..index];
            let right_slice = &nums[index+1..];
            if left_slice.iter().sum::<i32>() == right_slice.iter().sum::<i32>()
            {
                return index as i32;
            }
            index += 1;
        }
        return -1;
    }
}
