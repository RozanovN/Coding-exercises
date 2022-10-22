// Given an array nums. We define a running sum of an array as runningSum[i] = sum(nums[0]…nums[i]).
//
// Return the running sum of nums.
// https://leetcode.com/problems/running-sum-of-1d-array/

impl Solution {
    pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
        let mut index = 1;
        while index < nums.len()
        {
            nums[index] += nums[index - 1];
            index += 1;
        }
        return nums;
    }
}
