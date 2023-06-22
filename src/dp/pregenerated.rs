use crate::dp::problems::Problem;
use num::BigUint;
use num::Zero;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

pub struct PregeneratedSolution {
    table: Vec<Vec<Vec<BigUint>>>,
    time_limit: usize,
    data_path: String,
}

impl PregeneratedSolution {
    pub fn new(time_limit: usize, data_path: String) -> Self {
        Self {
            table: vec![
                vec![vec![Zero::zero(); 2 * time_limit + 2]; 2 * time_limit + 2];
                time_limit + 1
            ],
            time_limit,
            data_path,
        }
    }

    fn limits(&self) -> (isize, isize) {
        (-(self.time_limit as isize), self.time_limit as isize)
    }

    fn set(&mut self, x: isize, y: isize, t: usize, val: BigUint) {
        let x = (self.time_limit as isize + x) as usize;
        let y = (self.time_limit as isize + y) as usize;

        self.table[t][x][y] = val;
    }

    pub fn table(&self) -> Vec<Vec<Vec<BigUint>>> {
        self.table.clone()
    }

    pub fn time_limit(&self) -> usize {
        self.time_limit
    }
}

impl Problem for PregeneratedSolution {
    /// For each cell, count the number of paths leading to it in each time step.
    fn count_paths(&mut self) {
        let (limit_neg, limit_pos) = self.limits();

        for t in 1..=limit_pos as usize {
            let mut path = PathBuf::from(&self.data_path);
            path.set_file_name(format!("c_{}", self.time_limit));

            let f = File::open(&path).expect(&format!(
                "Could not open file: {}.",
                &path.to_str().expect("Could not open file.")
            ));
            let mut reader = BufReader::new(f);

            for y in limit_neg..=limit_pos {
                let mut line = String::new();
                reader.read_line(&mut line).expect("Could not read line.");
                let mut xs = line.split_whitespace();

                for x in limit_neg..=limit_pos {
                    self.set(
                        x,
                        y,
                        t,
                        BigUint::from_str(xs.next().expect("Could not get next element."))
                            .expect("Could not parse value."),
                    );
                }
            }
        }
    }

    fn generate_path(&self, _x: isize, _y: isize, _t: usize) -> Vec<(isize, isize)> {
        todo!()
    }

    fn generate_path_bias(&self, _x: isize, _y: isize, _t: usize) -> Vec<(isize, isize)> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::dp::pregenerated::PregeneratedSolution;
    use crate::dp::problems::Problem;
    use crate::dp::DynamicProgram;

    #[test]
    fn testing() {
        let mut solution = PregeneratedSolution::new(10, String::from("counts"));
        solution.count_paths();

        let dp = DynamicProgram::from(solution);
        let path = dp.generate_path(2, 5, 10);

        println!("{:?}", path);

        assert_eq!(1, 1);
    }
}
