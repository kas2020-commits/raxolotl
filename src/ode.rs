
// The simplest method out there for approximating the solution to a
// differential equation.
// Local Error: O(h^2)
// Solution order: 1
pub fn euler
(h: f64, a: f64, b: f64, y0: f64, df: impl Fn(f64) -> f64) -> Vec<f64> {
    let mut x = a;
    let mut y = y0;
    let mut ans = Vec::with_capacity((1f64/h) as usize);
    while x < b {
        ans.push(y);
        y += h * df(y);
        x += h;
    }
    ans
}

// A simple modification to Euler's method which uses 2 function calls rather
// than 1, and takes its average.
// Local Error: O(h^3)
// Solution order: 2
pub fn trapezoidal
(h: f64, a: f64, b: f64, y0: f64, df: impl Fn(f64) -> f64) -> Vec<f64> {
    let mut x = a;
    let mut y = y0;
    let mut ans = Vec::with_capacity((1f64/h) as usize);
    while x < b {
        ans.push(y);
        y += h / 2f64 * ( df(y) + df(y + h) );
        x += h;
    }
    ans
}

