pub struct CircularArray<T: Clone> {
    elements: Vec<T>,
}

enum Direction {
    Forward,
    Backward,
}
pub struct CircularArrayIter<T: Clone> {
    elements: Vec<T>,
    idx: usize,
    direction: Direction,
    remaining: usize,
}


impl<T: Clone> CircularArray<T> {
    pub fn new(elements: &[T]) -> Self {
        Self {
            elements: elements.to_vec(),
        }
    }
    pub fn forward(&self, start_idx: usize) -> CircularArrayIter<T> {
        CircularArrayIter::new(&self.elements, start_idx, Direction::Forward)
    }
    pub fn backward(&self, start_idx: usize) -> CircularArrayIter<T> {
        CircularArrayIter::new(&self.elements, start_idx, Direction::Backward)
    }
}

impl <T: Clone> CircularArrayIter<T> {
    fn new(words: &[T], start_idx: usize, direction: Direction) -> Self {
        Self {
            idx: start_idx,
            elements: words.to_vec(),
            direction,
            remaining: words.len(),
        }
    }
}

impl <T: PartialEq + Clone> CircularArrayIter<T> {
    pub fn get_target(self, target: &T) -> Vec<usize> {
        self
            .enumerate()
            .filter(|(_, s)| s == target)
            .map(|(i, _)| i + 1)
            .collect()
    }
}

impl <T: Clone> Iterator for CircularArrayIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.idx;

        let n = self.elements.len();

        self.idx = match self.direction {
            Direction::Forward => (i + 1) % n,
            Direction::Backward => (i + n - 1) % n,
        };

        let curr = &self.elements[self.idx];

        if self.remaining == 0 {
            return None;
        }

        self.remaining -= 1;
        Some(curr.to_owned())
    }
}
