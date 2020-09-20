
type Num = f32;

/// The simplest Theta method (theta = 1) out there for approximating the
/// solution to a differential equation.
/// Local Error: O(h^2)
/// Solution order: 1
/// Type: Explicit
pub fn euler
(h: Num, a: Num, b: Num, y0: Num, df: impl Fn(Num) -> Num) -> Vec<Num> {
    let n = ((b - a) / h) as usize + 1;
    let mut y = y0;
    let mut ans = Vec::with_capacity(n);
    for _i in 0..n {
        ans.push(y);
        y += h * df(y);
    }
    ans
}

/// A simple modification to Euler's method (theta = 1/2) which uses 2 function
/// calls rather than 1, and takes its average.
/// Local Error: O(h^3)
/// Solution order: 2
/// Type: Implicit
/// General Form: Theta method
pub fn trapezoidal
(h: Num, a: Num, b: Num, y0: Num, df: impl Fn(Num) -> Num) -> Vec<Num> {
    let n = ((b - a) / h) as usize + 1;
    let mut y = y0;
    let mut ans = Vec::with_capacity(n);
    for _i in 0..n {
        ans.push(y);
        y += h / 2.0 * ( df(y) + df(y + h) );
    }
    ans
}

