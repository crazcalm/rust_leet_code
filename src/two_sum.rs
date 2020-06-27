/*
Given an array of integers, return indices of the two numbers
such that they add up to a specific target.

You may assume that each input would have exactly one solution,
and you may not use the same element twice.

Example:

Given nums = [2, 7, 11, 15], target = 9,

Because nums[0] + nums[1] = 2 + 7 = 9,
return [0, 1].
*/

#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut solution = vec![-1];

    for (index_x, x) in nums.iter().enumerate() {
        for (index_y, y) in nums.iter().enumerate() {
            if x + y == target && index_x != index_y {
                solution = vec![index_x as i32, index_y as i32];
                return solution;
            }
        }
    }
    solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sums() {
        let test_cases = vec![
            (vec![2, 7, 11, 15], 9, vec![0, 1]),
            (vec![3, 2, 4], 6, vec![1, 2]),
        ];

        for (list, target, expected) in test_cases {
            let result = two_sum(list, target);

            assert_eq!(result, expected);
        }
    }
}
