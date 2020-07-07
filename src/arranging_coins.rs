/*
You have a total of n coins that you want to form in a staircase shape,
where every k-th row must have exactly k coins.

Given n, find the total number of full staircase rows that can be
formed.

n is a non-negative integer and fits within the range of 32-bit
signed integer.

Example 1:
    n = 5

    The coins can form the following rows:
    x
    xx
    xx

    Because the 3rd row is incomplete, we return 2.


Example 2:
    n = 8

    x
    xx
    xxx
    xx

    Because the 4th row is incomplete, we return 3.
 */

#[allow(dead_code)]
pub fn solution(mut coins: u32) -> u32 {
    let mut count = 0;

    for num in 1..coins.clone() {
        if coins >= num {
            count+= 1;
            coins -= num;
        } else {
            break;
        }
    }

    count
}

mod tests {
    #[allow(unused)]
    use super::solution;

    #[test]
    fn test_solution(){
        #[derive(Debug)]
        struct TestCase {
            input: u32,
            expected: u32,
        };

        let cases = vec![
            TestCase{input: 5, expected: 2},
            TestCase{input: 8, expected: 3},
        ];

        for case in cases {
            let result = solution(case.input);
            assert_eq!(result, case.expected);
        }
    }
}
