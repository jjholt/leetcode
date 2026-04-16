#![allow(dead_code)]
struct Solution;

use crate::circular_iter::CircularArray;

impl Solution {
    pub fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let start_idx = usize::try_from(start_index).unwrap();

        if words[start_idx] == target {
            return 0
        }

        let idx_forward: Vec<usize> = CircularArray::new(&words)
            .forward(start_idx)
            .get_target(&target);

        let mut idx_reverse: Vec<usize> = CircularArray::new(&words)
            .backward(start_idx)
            .get_target(&target);

        let mut idx = idx_forward.clone();
        idx.append(&mut idx_reverse);

        match idx.iter().min() {
            Some(u) => i32::try_from(*u).unwrap(),
            None => -1,
        }
    }
}

#[cfg(test)]
mod shorted_distance {
    use super::*;

    #[test]
    fn case_1() {
        let words = ["hello", "i", "am", "leetcode", "hello"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let target = String::from("hello");
        let start_index = 1;
        let solution = Solution::closest_target(words, target, start_index);
        assert_eq!(solution, 1)
    }

    #[test]
    fn case_2() {
        let words = ["a", "b", "leetcode"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let target = String::from("leetcode");
        let start_index = 0;
        let solution = Solution::closest_target(words, target, start_index);
        assert_eq!(solution, 1)
    }

    #[test]
    fn case_3() {
        let words = ["i", "eat", "leetcode"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let target = String::from("ate");
        let start_index = 0;
        let solution = Solution::closest_target(words, target, start_index);
        assert_eq!(solution, -1)
    }

    #[test]
    fn case_4() {
        let words = ["ibkgecmeyx","jsdkekkjsb","gdjgdtjtrs","jsdkekkjsb","jsdkekkjsb","jsdkekkjsb","gdjgdtjtrs","msjlfpawbx","pbgjhutcwb","gdjgdtjtrs"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let target = String::from("pbgjhutcwb");
        let start_index = 0;
        let solution = Solution::closest_target(words, target, start_index);
        assert_eq!(solution, 2)
    }

    #[test]
    fn reverse_iter() {
        let words: Vec<String> = ["a", "b", "c", "d", "f"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let words_reversed: Vec<String> = CircularArray::new(&words)
            .backward(1)
            .collect();

        let byhand: Vec<String> = ["a", "f", "d", "c", "b"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        assert_eq!(words_reversed, byhand)
    }

    #[test]
    fn case_5() {
        let words = ["pgmiltbptl","jnkxwutznb","bmeirwjars","ugzyaufzzp","pgmiltbptl","sfhtxkmzwn","pgmiltbptl","pgmiltbptl","onvmgvjhxa","jyzdtwbwqp"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let target = String::from("pgmiltbptl");
        let start_index = 4;
        let solution = Solution::closest_target(words, target, start_index);
        assert_eq!(solution, 0)
    }
    
}
