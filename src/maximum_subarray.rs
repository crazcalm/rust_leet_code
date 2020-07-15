/*
Given an integer array nums, find the contigous
subarray (containing at least one number) which has
the largest sum and return its sum.

Example:
    Input: [-2, 1, -3, 4, -1, 2, 1, -5, 4],
    Output: 6
    Explanation: [4, -1, 2, 1] has the largest sum 6.

Follow up:
If you have figured out the O(n) solution, try coding
another solution using the divide and conquer approach,
which is more subtle.

Notes:
- recusive soltion is from the "Introduction to ALGORITHMS" book (Chapter 4.1 -- Divide-and-Conquer).
- Kadane's solution can be found on Wikipedia -- https://en.wikipedia.org/wiki/Maximum_subarray_problem
*/

use std::i32;

#[derive(Debug, PartialEq, Eq)]
pub struct Solution {
    start_index: usize,
    end_index: usize,
    sum: i32,
}

impl Solution {
    #[allow(dead_code)]
    fn new(start_index: usize, end_index: usize, sum: i32) -> Solution {
        Solution {
            start_index,
            end_index,
            sum,
        }
    }
}

#[allow(dead_code)]
pub fn brute_force(arr: &[i32]) -> Option<Solution> {
    let mut result = None;
    let mut sum = i32::MIN;

    for start in 0..arr.len() {
        let mut current_sum = 0;

        for end in start..arr.len() {
            current_sum += arr[end];

            if current_sum > sum {
                sum = current_sum;
                result = Some(Solution::new(start, end, sum));
            }
        }
    }

    result
}

#[allow(dead_code)]
pub fn divide_and_conquer(arr: &[i32], low: usize, high: usize) -> Option<Solution> {
    //Edge case for empty array
    if arr.is_empty() {
        return None;
    }

    match high == low {
        true => Some(Solution::new(low, high, arr[low])),
        false => {
            let mid = (low + high) / 2;

            let left_result = divide_and_conquer(arr, low, mid).unwrap();
            let right_result = divide_and_conquer(arr, mid + 1, high).unwrap();

            let middle_result = find_max_crossing_subarray(arr, low, mid, high);

            if left_result.sum >= right_result.sum && left_result.sum >= middle_result.sum {
                Some(left_result)
            } else if right_result.sum >= left_result.sum && right_result.sum >= middle_result.sum {
                Some(right_result)
            } else {
                Some(middle_result)
            }
        }
    }
}

#[allow(dead_code)]
fn find_max_crossing_subarray(arr: &[i32], low: usize, mid: usize, high: usize) -> Solution {
    let mut left_sum = i32::MIN;
    let mut sum = 0;
    let mut max_left = 0;

    for index in (low..=mid).rev() {
        sum += arr[index];

        if sum > left_sum {
            left_sum = sum;
            max_left = index;
        }
    }

    let mut right_sum = i32::MIN;
    sum = 0;
    let mut max_right = 0;

    for index in mid + 1..=high {
        sum += arr[index];

        if sum > right_sum {
            right_sum = sum;
            max_right = index;
        }
    }

    Solution::new(max_left, max_right, left_sum + right_sum)
}

#[allow(dead_code)]
pub fn kadane(arr: &[i32]) -> Option<Solution> {
    let mut best_sum = i32::MIN;
    let mut best_start = 0;
    let mut best_end = 0;
    let mut current_start = 0;
    let mut current_sum = 0;

    //Edge Case
    if arr.is_empty() {
        return None;
    }

    for (current_end, x) in arr.iter().enumerate() {
        if current_sum + *x <= *x {
            //Start a new sequence at the current element
            current_start = current_end;
            current_sum = *x;
        } else {
            // Extend the existing sequence with the current element
            current_sum += x;

            if current_sum > best_sum {
                best_sum = current_sum;
                best_start = current_start;
                best_end = current_end;
            }
        }
    }

    Some(Solution::new(best_start, best_end, best_sum))
}

#[cfg(test)]
mod tests {
    use super::{brute_force, divide_and_conquer, kadane, Solution};

    struct TestCase {
        input: Vec<i32>,
        expected: Solution,
    }

    impl TestCase {
        fn new(input: Vec<i32>, expected: Solution) -> TestCase {
            TestCase { input, expected }
        }
    }

    fn get_test_cases() -> Vec<TestCase> {
        vec![
            TestCase::new(vec![1, 2, 3, -1, -2, -3, 4, 1], Solution::new(0, 2, 6)),
            TestCase::new(vec![1, 2, 3, -2, -3, -4, 4, 3], Solution::new(6, 7, 7)),
            TestCase::new(vec![-1, -1, 2, 3, 4, -1, -1, -1], Solution::new(2, 4, 9)),
            TestCase::new(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4], Solution::new(3, 6, 6)),
        ]
    }

    #[test]
    fn test_brute_force() {
        //Normal cases
        for case in get_test_cases() {
            let result = brute_force(&case.input[..]);
            assert_eq!(result.unwrap(), case.expected);
        }

        //Edge Case
        let empty_vec: Vec<i32> = Vec::new();
        assert_eq!(None, brute_force(&empty_vec[..]));
    }

    #[test]
    fn test_divide_and_conquer() {
        //Normal cases
        for (num, case) in get_test_cases().iter().enumerate() {
            let result = divide_and_conquer(&case.input[..], 0, case.input.len() - 1);
            assert_eq!(result.unwrap(), case.expected, "See Case num {}", num + 1);
        }

        //Edge Case
        let empty_vec: Vec<i32> = Vec::new();
        assert_eq!(None, divide_and_conquer(&empty_vec[..], 0, 0))
    }

    #[test]
    fn test_kadane() {
        //Normal cases
        for (num, case) in get_test_cases().iter().enumerate() {
            let result = kadane(&case.input[..]);
            assert_eq!(result.unwrap(), case.expected, "See Case num {}", num + 1);
        }

        //Edge Case
        let empty_vec: Vec<i32> = Vec::new();
        assert_eq!(None, kadane(&empty_vec[..]));
    }
}
