struct Solution;

struct WordsForward {
    arr: Vec<String>,
    idx: usize,
}
struct WordsBackward {
    arr: Vec<String>,
    idx: usize,
}

struct Words {
    arr: Vec<String>,
}

impl Words {
    pub fn new(words: &[String]) -> Self {
        Self {
            arr: words.to_vec(),
        }
    }
    pub fn forward(&self, start_idx: usize) -> WordsForward {
        WordsForward::new(&self.arr, start_idx)
    }
    pub fn backward(&self, start_idx: usize) -> WordsBackward {
        WordsBackward::new(&self.arr, start_idx)
    }
}

impl WordsForward {
    fn new(words: &[String], start_idx: usize) -> Self {
        Self {
            idx: start_idx,
            arr: words.to_vec(),
        }
    }
    fn get_target(self, target: &str) -> Vec<usize> {
        self
            .enumerate()
            .filter(|(i, s)| s == target)
            .map(|(i, _)| i)
            .collect()
    }
}
impl WordsBackward {
    fn new(words: &[String], start_idx: usize) -> Self {
        Self {
            idx: start_idx,
            arr: words.to_vec(),
        }
    }
    fn get_target(self, target: &str) -> Vec<usize> {
        self
            .enumerate()
            .filter(|(i, s)| s == target)
            .map(|(i, _)| i)
            .collect()
    }
}

impl Iterator for WordsForward {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = &self.arr[(self.idx + 1) % self.arr.len()];
        self.idx += 1;
        if self.idx > self.arr.len() {
            return None;
        }
        Some(curr.to_string())
    }
}
impl Iterator for WordsBackward {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.idx;
        let n = self.arr.len();
        let curr = &self.arr[(i + n - 1) % n];
        self.idx += 1;
        if self.idx > self.arr.len() {
            return None;
        }
        Some(curr.to_string())
    }
}

impl Solution {
    pub fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let start_idx = usize::try_from(start_index).unwrap();
        let idx_forward: Vec<usize> = Words::new(&words)
            .forward(start_idx)
            .get_target(&target);

        let mut idx_reverse: Vec<usize> = Words::new(&words)
            .backward(start_idx)
            .get_target(&target);

        let mut idx = idx_forward.clone();
        idx.append(&mut idx_reverse);

        match idx.iter().min() {
            Some(u) => i32::try_from(*u).unwrap() + 1,
            None => -1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        println!("Case 1");
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
        println!("Case 2");
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
        println!("Case 3");
        let words = ["i", "eat", "leetcode"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let target = String::from("ate");
        let start_index = 0;
        let solution = Solution::closest_target(words, target, start_index);
        assert_eq!(solution, -1)
    }
}
