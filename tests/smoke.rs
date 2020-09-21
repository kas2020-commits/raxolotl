#[cfg(test)]
extern crate raxolotl;

#[test]
fn numerical() {
    type Num = f64;
    let f = |x: Num| { x.powf(3.0) - x - 2.0 };
    let df = |x: Num| { (3.0 * x.powf(2.0)) - 1.0 };
    let tol = 1e-7;
    let max_it = 1000;
    let nm = (
        raxolotl::newton_raphson(tol, max_it, &f, &df, 2.0),
        raxolotl::bisection(tol, max_it, &f, 0.0, 2.0),
        raxolotl::secent(tol, max_it, &f, 0.0, 2.0)
    );
    let answer = 1.5213797;
    assert!((nm.0.0 - answer).abs() < 1e-7);
    assert!((nm.1.0 - answer).abs() < 1e-7);
    assert!((nm.2.0 - answer).abs() < 1e-7);
}

#[test]
fn integral() {
    type Num = f64;
    let max_it = 1000;
    let tol = 5.0;
    let a = 0.0;
    let b = 10.0;
    let f2 = |x: Num| { x.exp() };
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
    let mat1 = raxolotl::Matrix::new(vec![vec![1.0, 2.0, 8.0], vec![3.0, 4.0, 6.0]]);
    assert!(mat1.at(1,2) == 6.0);
}

#[test]
fn ode() {
    type Num = f32;
    // closures
    let odf = |x: Num| { 1.0 * x };
    let fc = |x: Num| { x.exp() };
    // calculate the control vector as well as testings
    let control = raxolotl::calc_real_sol(0.01, 0.0, 1.0, fc);
    let r_euler = raxolotl::euler(0.01, 0.0, 1.0, 1.0, odf);
    let r_trap = raxolotl::trapezoidal(0.01, 0.0, 1.0, 1.0, odf);
    // asserts
    assert!(raxolotl::check_vec_tol(&control, &r_euler, 0.1));
    assert!(raxolotl::check_vec_tol(&control, &r_trap, 0.1));
}
