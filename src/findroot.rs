
//! Perhaps the most simple algorithm to find the root of a function. The problem
//! with the newton_raphson method, however, is the fact that you need to know
//! the derivative function as well.
pub fn newton_raphson
(tol: f64, max_it: u32, f: impl Fn(f64) -> f64, df: impl Fn(f64) -> f64, mut x0: f64) -> (f64, u32) {
    for cur_it in 1..max_it {
        let h = f(x0) / df(x0);
        let retval = x0 - h;
        if h.abs() < tol { return (retval, cur_it) };
        x0 = retval;
    }
    panic!("max itterations reached")
}

pub fn bisection
(tol: f64, max_it: u32, f: impl Fn(f64) -> f64, mut a: f64, mut b: f64) -> (f64, u32) {
    for cur_it in 1..max_it {
        let retval = (a + b) / 2f64;
        let h = f(retval);
        if h.abs() < tol { return (retval, cur_it) };
        if h.signum() == f(a).signum() {
            a = retval
        } else { b = retval };
    }
    panic!("max itterations reached")
}

pub fn secent
(tol: f64, max_it: u32, f: impl Fn(f64) -> f64, mut x0: f64, mut x1: f64) -> (f64, u32) {
    for cur_it in 1..max_it {
        let h = f(x0) * ((x1 - x0) / (f(x1) - f(x0)));
        let retval = x0 - h;
        if h.abs() < tol { return (retval, cur_it) };
        x1 = x0;
        x0 = retval;
    }
    panic!("max itterations reached")
}

