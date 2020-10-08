
type Num = f64;

/// Possibly the simplest algorithm for finding the root of a function. This is
/// Decent if you already have the analytical derivative known.
pub fn root_newton_raphson
(tol: Num, max_it: u32, f: impl Fn(Num) -> Num, df: impl Fn(Num) -> Num, mut x0: Num) -> (Num, u32) {
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
(tol: Num, max_it: u32, f: impl Fn(Num) -> Num, mut a: Num, mut b: Num) -> (Num, u32) {
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
(tol: Num, max_it: u32, f: impl Fn(Num) -> Num, mut x0: Num, mut x1: Num) -> (Num, u32) {
    for cur_it in 1..max_it {
        let h = f(x0) * ((x1 - x0) / (f(x1) - f(x0)));
        let retval = x0 - h;
        if h.abs() < tol { return (retval, cur_it) };
        x1 = x0;
        x0 = retval;
    }
    panic!("max iterations reached")
}

