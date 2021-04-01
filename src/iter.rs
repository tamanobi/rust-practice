#[derive(Debug)]
pub struct Counter {
    length: usize,
    count: usize,
}

impl Counter {
    pub fn new(length: usize) -> Counter {
        Counter {
            count: 0,
            length,
        }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count <= self.length {
            Some(self.count)
        } else {
            None
        }
    }
}
