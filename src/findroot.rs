
/// Possibly the simplest algorithm for finding the root of a function. This is
/// Decent if you already have the analytical derivative known.
pub fn root_newton_raphson
(tol: f64, max_it: u32, f: impl Fn(f64) -> f64, df: impl Fn(f64) -> f64, mut x0: f64) -> (f64, u32) {
    for cur_it in 1..max_it {
        let h = f(x0) / df(x0);
        let retval = x0 - h;
        if h.abs() < tol { return (retval, cur_it) };
        x0 = retval;
    }
    panic!("max iterations reached")
}

/// An extremely basic algorithm where the solution to the first derivative is
/// Not needed. Linear convergence is almost always satisfied, however linear
/// Convergence sucks. Ideal algorithm for simplicity's sake but not much else.
pub fn root_bisection
(tol: f64, max_it: u32, f: impl Fn(f64) -> f64, mut a: f64, mut b: f64) -> (f64, u32) {
    for cur_it in 1..max_it {
        let retval = (a + b) / 2.0;
        let h = f(retval);
        if h.abs() < tol { return (retval, cur_it) };
        if h.signum() == f(a).signum() {
            a = retval
        } else { b = retval };
    }
    panic!("max iterations reached")
}

/// A method with the same interface as bisection but is more related to newton.
/// This method approximates the derivative at every step (linear approx). This
/// Converges faster than bisection.
pub fn root_secent
(tol: f64, max_it: u32, f: impl Fn(f64) -> f64, mut x0: f64, mut x1: f64) -> (f64, u32) {
    for cur_it in 1..max_it {
        let h = f(x0) * ((x1 - x0) / (f(x1) - f(x0)));
        let retval = x0 - h;
        if h.abs() < tol { return (retval, cur_it) };
        x1 = x0;
        x0 = retval;
    }
    panic!("max iterations reached")
}

