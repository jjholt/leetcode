use std::collections::HashSet;
use crate::circular_iter::CircularArray;

struct Solution;

impl Solution {
    pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
         queries
            .iter()
            .map(|q| *q as usize)
            .map(|i| (i, nums[i]))
            .map(|(i, target)| {
                let mut forward: Vec<_> = CircularArray::new(&nums).forward(i).get_target(&target);


                let backward: Vec<_> = CircularArray::new(&nums).backward(i).get_target(&target);

                forward.extend(backward);
                forward.into_iter().collect::<HashSet<usize>>()
            })
            .map(|f| {
                let minima = f.into_iter().min().map(|f| f as i32);
                if minima == Some(nums.len() as i32) {
                    return None
                };
                minima
            })
            .map(|f| f.unwrap_or(-1))
            // .inspect(|f| println!("{:?}", f))
            .collect()
        
    }
}
#[cfg(test)]
mod test {
    #[test]
    fn case_1() {
        let nums = [1, 3, 1, 4, 1, 3, 2];
        let queries = [0, 3, 5];

        let solution = super::Solution::solve_queries(nums.to_vec(), queries.to_vec());
        assert_eq!(solution, [2,-1,3])
    }

    #[test]
    fn case_2() {
        let nums = [1,2,3,4];
        let queries = [0,1,2,3];

        let solution = super::Solution::solve_queries(nums.to_vec(), queries.to_vec());
        assert_eq!(solution, [-1, -1, -1, -1])
    }
}
