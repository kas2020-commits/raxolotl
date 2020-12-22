
type Num = f32;

/// Type: Explicit
pub fn odesolve_euler
(tspan: &[Num], y0: &Vec<Num>, df: impl Fn(Num, &[Num]) -> Vec<Num>) -> Vec<Vec<Num>> {
    let n = tspan.len();
    let h = tspan[1] - tspan[0];
    let dim = y0.len();
    let mut y = Vec::with_capacity(n);
    let mut y_prev = y0.clone();
    for i in 0..n {
        y.push( Vec::with_capacity(dim) );
        let inter = df(0., &y_prev);
        for k in 0..dim {
            y[i].push( y_prev[k] + h*inter[k] );
        }
        y_prev = y[i].clone();
    }
    y
}

/// Type: Explicit
pub fn odesolve_erk4
(tspan: &[Num], y0: &Vec<Num>, df: impl Fn(Num, &[Num]) -> Vec<Num>) -> Vec<Vec<Num>> {
    let n = tspan.len();
    let h = tspan[1] - tspan[0];
    let dim = y0.len();
    let mut y = Vec::with_capacity(n);
    let mut y_prev = y0.clone();
    for i in 0..n {
        y.push(Vec::with_capacity(dim));
        // calculate the k's
        let k1 = y_prev.clone();
        let d1 = df(tspan[i], &k1);
        let k2 = k1.iter().zip(&d1).map(|(a,b)| a + (h/2.)*b).collect::<Vec<_>>();
        let d2 = df(tspan[i] + (h/2.), &k2);
        let k3 = k1.iter().zip(&d2).map(|(a,b)| a + (h/2.)*b).collect::<Vec<_>>();
        let d3 = df(tspan[i] + h, &k2);
        let k4 = k1.iter().zip(&d3).map(|(a,b)| a + h*b).collect::<Vec<_>>();
        // calculate intermediate values
        let d4 = df(tspan[i], &k1);
        let p1 = d4.iter().map(|a| (h/6.)*a).collect::<Vec<_>>();
        let d5 = df(tspan[i] + h/2., &k2);
        let p2 = d5.iter().map(|a| (h/3.)*a).collect::<Vec<_>>();
        let d6 = df(tspan[i] + h/2., &k3);
        let p3 = d6.iter().map(|a| (h/3.)*a).collect::<Vec<_>>();
        let d7 = df(tspan[i] + h, &k4);
        let p4 = d7.iter().map(|a| (h/6.)*a).collect::<Vec<_>>();
        // constructs the vector for a given timestep
        for k in 0..dim {
            y[i].push( y_prev[k] + p1[k] + p2[k] + p3[k] + p4[k]);
        }
        y_prev = y[i].clone();
    }
    y
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

