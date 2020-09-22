
type Num = f32;

/// a quick way to create a vector of results for a given candidate solution
pub fn calc_real_sol
(h: Num, a: Num, b: Num, f: impl Fn(Num) -> Num) -> Vec<Num> {
    let n = ((b - a) / h) as usize + 1;
    let mut x = a;
    let mut ans = Vec::with_capacity(n);
    for _i in 0..n {
        ans.push(f(x));
        x += h;
    }
    ans
}

/// a simple checker of tolerance between 2 vectors of numbers
/// panics if check fails
pub fn check_vec_tol
(v1: &Vec<Num>, v2: &Vec<Num>, tol: Num) -> bool {
    assert!(v1.len() == v2.len());
    let len = v1.len();
    for i in 0..len {
        if (v1[i] - v2[i]).abs() < tol {
            continue;
        } else {
            return false;
        }
    }
    true
}

/// The simplest Theta method (theta = 1) out there for approximating the
/// solution to a differential equation.
/// Local Error: O(h^2)
/// Solution order: 1
/// Type: Explicit
pub fn odesolve_euler
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
pub fn odesolve_trapezoidal
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

