type Num = f32;

/// A simple modification to Euler's method (theta = 1/2) which uses 2 function
/// calls rather than 1, and takes its average.
/// Local Error: O(h^3)
/// Solution order: 2
/// Type: Implicit
/// General Form: Theta method
pub fn odesolve_trapezoidal
(h: Num, a: Num, b: Num, y0: Num, df: impl Fn(Num) -> Num) -> Vec<Num> {
    let n = ((b - a) / h) as usize + 1;
    let mut y = Vec::with_capacity(n);
    y.push(y0);
    for i in 1..n {
        y.push( y[i-1] + (h/2. * (df(y[i-1]) + df(y[i-1]+h))) );
    }
    y
}

/// Type: Explicit
pub fn odesolve_nystrom
(h: Num, a: Num, b: Num, y0: Num, df: impl Fn(Num) -> Num) -> Vec<Num> {
    let n = ((b - a) / h) as usize + 1;
    let mut y = Vec::with_capacity(n);
    y.push(y0);
    y.push(y0 + h * df(y0));
    for i in 2..n {
        y.push( y[i-2] + 2. * h * df(y[i-1]) );
    }
    y
}

