#[derive(Default)]
pub struct IsingModel {
    num_spins: usize,
    local_field: Vec<f64>,
    couplings: Vec<f64>,
}

impl IsingModel {
    pub fn load_qubo(&mut self, num_spins: usize, qubo_matrix: &[f64]) {
        self.num_spins = num_spins;
        self.calculate_local_fields(qubo_matrix);
        self.calculate_couplings(qubo_matrix);
    }

    fn calculate_local_fields(&mut self, qubo_matrix: &[f64]) {
        self.local_field = vec![0.0; self.num_spins];
        for i in 0..self.num_spins {
            let matrix_slice = &qubo_matrix[self.num_spins * i..self.num_spins * (i + 1)];
            self.local_field[i] = matrix_slice[i] / 2.0f64
                + 0.25f64
                    * (0..self.num_spins)
                        .map(|j| if i == j { 0.0f64 } else { matrix_slice[j] })
                        .sum::<f64>();
        }
    }

    fn calculate_couplings(&mut self, qubo_matrix: &[f64]) {
        self.couplings = qubo_matrix.iter().map(|v| v / 4.0).collect();
    }
}
