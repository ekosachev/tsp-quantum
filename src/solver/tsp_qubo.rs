#[derive(Default)]
pub struct QUBO {
    pub size: usize,
    pub state: Vec<bool>,
}

impl QUBO {
    pub fn with_size(mut self, size: usize) -> Self {
        self.size = size;
        self
    }
}
