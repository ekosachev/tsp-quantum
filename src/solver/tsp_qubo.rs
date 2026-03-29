#[derive(Default)]
pub struct QUBO {
    size: usize,
}

impl QUBO {
    pub fn with_size(mut self, size: usize) -> Self {
        self.size = size;
        self
    }
}
