#[derive(Default)]
pub struct QUBO {
    pub num_cities: usize,
    pub state: Vec<f64>,
    weights: QuboWeights,
}

struct QuboWeights {
    visit_once: f64,
    one_city_in_position: f64,
}

impl Default for QuboWeights {
    fn default() -> Self {
        Self {
            visit_once: 1.0,
            one_city_in_position: 1.0,
        }
    }
}

impl QUBO {
    pub fn num_variables(&self) -> usize {
        self.num_cities.pow(2)
    }

    fn get_state_index(&self, u: usize, v: usize) -> usize {
        self.num_variables() * u + v
    }

    fn get_var_index(&self, i: usize, t: usize) -> usize {
        self.num_cities * i + t
    }

    fn add_linear(&mut self, variable: usize, weight: f64) {
        let index = self.get_state_index(variable, variable);
        self.state[index] += weight;
    }

    fn add_quadratic(&mut self, var_a: usize, var_b: usize, weight: f64) {
        let index_a = self.get_state_index(var_a, var_b);
        let index_b = self.get_state_index(var_b, var_a);

        self.state[index_a] += weight;
        self.state[index_b] += weight;
    }

    pub fn update_weights(&mut self) {
        self.state = vec![0.0; self.num_variables().pow(2)];

        self.visit_once_constraint();
        self.one_city_in_position_constraint();
    }

    fn visit_once_constraint(&mut self) {
        for i in 0..self.num_cities {
            for t in 0..self.num_cities {
                let var_index = self.get_var_index(i, t);
                self.add_linear(var_index, -self.weights.visit_once);

                for t2 in 0..t {
                    let other_var_index = self.get_var_index(i, t2);
                    self.add_quadratic(var_index, other_var_index, 2.0 * self.weights.visit_once);
                }
            }
        }
    }

    fn one_city_in_position_constraint(&mut self) {
        for t in 0..self.num_cities {
            for i in 0..self.num_cities {
                let var_index = self.get_var_index(i, t);
                self.add_linear(var_index, -self.weights.one_city_in_position);

                for i2 in 0..i {
                    let other_var_index = self.get_var_index(i2, t);
                    self.add_quadratic(
                        var_index,
                        other_var_index,
                        2.0 * self.weights.one_city_in_position,
                    );
                }
            }
        }
    }
}
