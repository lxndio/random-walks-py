pub struct DynamicProgram {
    table: Vec<Vec<Vec<usize>>>,
    time_limit: usize,
}

type WalkFunction = fn(&DynamicProgram, isize, isize, usize) -> usize;

impl DynamicProgram {
    pub fn new(time_limit: usize) -> Self {
        Self {
            table: vec![vec![vec![0; 2 * time_limit + 2]; 2 * time_limit + 2]; time_limit + 1],
            time_limit,
        }
    }

    pub fn limits(&self) -> (isize, isize) {
        (-(self.time_limit as isize), self.time_limit as isize)
    }

    pub fn get(&self, x: isize, y: isize, t: usize) -> usize {
        let x = (self.time_limit as isize + x) as usize;
        let y = (self.time_limit as isize + y) as usize;

        self.table[t][x][y]
    }

    pub fn set(&mut self, x: isize, y: isize, t: usize, val: usize) {
        let x = (self.time_limit as isize + x) as usize;
        let y = (self.time_limit as isize + y) as usize;

        self.table[t][x][y] = val;
    }

    pub fn update(&mut self, x: isize, y: isize, t: usize, walk: WalkFunction) {
        self.set(x, y, t, walk(self, x, y, t - 1));
    }

    pub fn print(&self, t: usize) {
        // Get number of digits of largest number
        let max = *self.table[t].iter().flatten().max().unwrap();
        let max_digits = max.to_string().len();

        for y in 0..2 * self.time_limit + 2 {
            for x in 0..2 * self.time_limit + 2 {
                let val = self.table[t][x][y];
                let digits = val.to_string().len();
                let spaces = " ".repeat(max_digits - digits + 2);

                print!("{}{}", val, spaces);
            }

            println!();
        }
    }
}
