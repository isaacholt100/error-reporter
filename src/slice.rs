pub trait Slice {
    fn slice(&self, start: usize, end: usize) -> String;
    fn slice_to_end(&self, start: usize) -> String;
}

impl Slice for str {
    fn slice(&self, start: usize, end: usize) -> String {
        self.chars().skip(start).take(end - start).collect()
    }
    fn slice_to_end(&self, start: usize) -> String {
        self.chars().skip(start).take(self.len() - start).collect()
    }
}