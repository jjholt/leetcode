// use crate::circular_iter::CircularArray;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut positions: HashMap<i32, Vec<usize>> = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            positions.entry(num).or_default().push(i);
        }

        queries
            .iter()
            .map(|&q| {
                let i = q as usize;
                let target = nums[i];
                let pos = &positions[&target];
                if pos.len() == 1 {
                    return -1
                }

                let idx = pos.binary_search(&i).unwrap();
                let mut best = usize::MAX;
                let next = pos[(idx+1) % pos.len()];
                let dist = (next + n - i) % n;
                best = best.min(dist);

                let prev = pos[(idx + pos.len() - 1) % pos.len()];
                let dist = (i + n - prev) % n;
                best = best.min(dist);
                best as i32
            })
            .collect()
    }
}
#[cfg(test)]
mod test {
    // #[test]
    // fn case_1() {
    //     let nums = [1, 3, 1, 4, 1, 3, 2];
    //     let queries = [0, 3, 5];
    //
    //     let solution = super::Solution::solve_queries(nums.to_vec(), queries.to_vec());
    //     assert_eq!(solution, [2, -1, 3])
    // }

    #[test]
    fn case_2() {
        let nums = [1, 2, 3, 4];
        let queries = [0, 1, 2, 3];

        let solution = super::Solution::solve_queries(nums.to_vec(), queries.to_vec());
        assert_eq!(solution, [-1, -1, -1, -1])
    }
}
