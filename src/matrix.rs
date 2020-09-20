
// TODO: Add more class for optimized calculations. A good start would be a
// sparse matrix class and a custom vector class.

/// Generic matrix class for computational methods relating to linear algebra.
pub struct Matrix
{
    data: Vec<Vec<f64>>,
}

impl Matrix
{
    /// custom constructor.
    pub fn new
        (arg: Vec<Vec<f64>>) -> Self {
            Matrix {
                data: arg
            }
    }

    /// prints the matrix to standard output using println.
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

    /// grabs an immutable copy of the number stored at a specific location in
    /// the matrix.
    pub fn at
        (&self, row: usize, col: usize) -> f64 {
            self.data[row][col]
    }
}

