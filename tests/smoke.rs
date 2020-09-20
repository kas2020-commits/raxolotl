#[cfg(test)]
extern crate raxolotl;

#[test]
fn numerical() {
    let f = |x: f64| { x.powf(3f64) - x - 2f64 };
    let df = |x: f64| { (3f64 * x.powf(2f64)) - 1f64 };
    let tol = 1e-7;
    let max_it = 1000;
    let nm = (
        raxolotl::newton_raphson(tol, max_it, &f, &df, 2.0),
        raxolotl::bisection(tol, max_it, &f, 0f64, 2f64),
        raxolotl::secent(tol, max_it, &f, 0f64, 2f64)
    );
    let answer = 1.5213797;
    assert!((nm.0.0 - answer).abs() < 1e-7);
    assert!((nm.1.0 - answer).abs() < 1e-7);
    assert!((nm.2.0 - answer).abs() < 1e-7);
}

#[test]
fn integral() {
    let max_it = 1000;
    let tol = 5f64;
    let a = 0f64;
    let b = 10f64;
    let f2 = |x: f64| { x.exp() };
    let nm = (
        raxolotl::composite_trapezoid(max_it, &f2, a, b),
        raxolotl::composite_midpoint(max_it, &f2, a, b),
        raxolotl::composite_simpson(max_it, &f2, a, b),
    );
    let answer = 22025.46579;
    assert!((nm.0 - answer).abs() < tol);
    assert!((nm.1 - answer).abs() < tol);
    assert!((nm.2 - answer).abs() < tol);
}

#[test]
fn matrix() {
    let mat1 = raxolotl::Matrix::new(vec![vec![1f64, 2f64, 8f64], vec![3f64, 4f64, 6f64]]);
    assert!(mat1.at(1,2) == 6f64);
}
