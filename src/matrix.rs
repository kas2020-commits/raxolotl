
pub struct Matrix
{
    pub rows: u32,
    pub cols: u32,
    pub data: Vec<Vec<f64>>,
}

impl Matrix
{
    pub fn print(&self) {
        for r in self.data.iter() {
            print!("[ ");
            for c in r.iter() {
                print!("{} ", c);
            }
            println!("]");
        }
    }
}
