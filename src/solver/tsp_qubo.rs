#[derive(Default)]
pub struct QUBO {
    pub num_cities: usize,
    pub state: Vec<f32>,
}

impl QUBO {
    pub fn num_variables(&self) -> usize {
        self.num_cities.pow(2)
    }
}
