/*
There are 8 prison cells in a row, and each cell is either
occupied or vacant.

Each day, whther the cell is occupied or vacant changes
according to the following rules:

    - If a cell has two adjacent neighbors that are both
      vacant, then the cell becomes occupied.

    - Otherwise, it becomes vacant.

    (Note that because the prison is a row, the first and
the last cells in the row can't have two adjacent neighbors.)

We describe the current state of the prison in the following way:
    cells[i] == 1 if the i-th cell is occupied, else cells[i] == 0

Given the initial state of the prison, return the state of
the presion after N days (and N such changes described above).

Example 1:
    Input: cells = [0,1,0,1,1,0,0,1], N = 7
    Output: [0,0,1,1,0,0,0,0]
    Explanation:
        The following table summarizes the state of the
        prison on each day:

        Day 0: [0, 1, 0, 1, 1, 0, 0, 1]
        Day 1: [0, 1, 1, 0, 0, 0, 0, 0]
        Day 2: [0, 0, 0, 0, 1, 1, 1, 0]
        Day 3: [0, 1, 1, 0, 0, 1, 0, 0]
        Day 4: [0, 0, 0, 0, 0, 1, 0, 0]
        Day 5: [0, 1, 1, 1, 0, 1, 0, 0]
        Day 6: [0, 0, 1, 0, 1, 1, 0, 0]
        Day 7: [0, 0, 1, 1, 0, 0, 0, 0]

Example 2:
    Input: cells = [1,0,0,1,0,0,1,0], N = 1000000000
    Output: [0,0,1,1,1,1,1,0]

 */

#[allow(dead_code)]
fn solution(mut cells: Vec<i32>, n: i32) -> Vec<i32> {
    let end_range = cells.len() - 1;

    for _ in 0..n {
        let mut tempt = vec![0; cells.len()];

        for x in 1..end_range {
            if cells[x - 1] == cells[x + 1] {
                tempt[x] = 1;
            } else {
                tempt[x] = 0;
            }
        }

        cells = tempt;
    }

    cells
}

#[allow(dead_code)]
#[allow(unused_mut)]
fn solution_2(mut cells: Vec<i32>, n: i32) -> Vec<i32> {
    //The trick is to realized that the possible
    //solution repeate after the 14th day.

    let mut days = 14;
    if n % 14 != 0 {
        days = n % 14;
    }

    solution(cells, days)
}

mod test {
    #[allow(unused)]
    use super::solution_2;

    #[test]
    fn test_solution() {
        struct TestCase {
            input: Vec<i32>,
            n: i32,
            expected: Vec<i32>,
        };

        let cases = vec![
            TestCase {
                input: vec![0, 1, 0, 1, 1, 0, 0, 1],
                n: 7,
                expected: vec![0, 0, 1, 1, 0, 0, 0, 0],
            },
            TestCase {
                input: vec![1, 0, 0, 1, 0, 0, 1, 0],
                n: 1000000000,
                expected: vec![0, 0, 1, 1, 1, 1, 1, 0],
            },
        ];

        for case in cases {
            let result = solution_2(case.input, case.n);

            assert_eq!(result, case.expected);
        }
    }
}
