
pub struct Matrix
{
    data: Vec<Vec<f64>>,
}

impl Matrix
{
    pub fn new
        (arg: Vec<Vec<f64>>) -> Self {
            Matrix {
                data: arg
            }
    }

    pub fn print
        (&self) {
            for r in self.data.iter() {
                print!("[ ");
                for c in r.iter() {
                    print!("{} ", c);
                }
                println!("]");
            }
    }

    pub fn at
        (&self, row: usize, col: usize) -> f64 {
            self.data[row][col]
    }
}

