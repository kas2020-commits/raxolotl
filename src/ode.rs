type Num = f64;

/// Creates a linear vector of equidistant numbers
pub fn linspace
(a: Num, b: Num, n: usize) -> Vec<Num> {
    let mut y = Vec::with_capacity(n);
    let h = (b - a) / n as Num;
    y.push(a);
    for i in 0..n {
        y.push(y[i] + h);
    }
    y
}

/// Type: Explicit
pub fn odesolve_euler
(tspan: &[Num], y0: &Vec<Num>, df: impl Fn(Num, &[Num]) -> Vec<Num>) -> Vec<Vec<Num>> {
    let n = tspan.len();
    let h = tspan[1] - tspan[0];
    let dim = y0.len();
    let mut y = Vec::with_capacity(n);
    let mut y_prev = y0.clone();
    for i in 0..n {
        y.push(Vec::with_capacity(dim));
        let inter = df(tspan[i], &y_prev);
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
        let k2 = k1.iter()
            .zip(df(tspan[i], &k1))
            .map(|(a,b)| a + (h/2.)*b).collect::<Vec<_>>();
        let k3 = k1.iter()
            .zip(df(tspan[i] + (h/2.), &k2))
            .map(|(a,b)| a + (h/2.)*b).collect::<Vec<_>>();
        let k4 = k1.iter()
            .zip(df(tspan[i] + h, &k3))
            .map(|(a,b)| a + h*b).collect::<Vec<_>>();
        // calculate intermediate values
        let p1 = df(tspan[i], &k1)
            .iter().map(|a| (h/6.)*a).collect::<Vec<_>>();
        let p2 = df(tspan[i] + h/2., &k2)
            .iter().map(|a| (h/3.)*a).collect::<Vec<_>>();
        let p3 = df(tspan[i] + h/2., &k3)
            .iter().map(|a| (h/3.)*a).collect::<Vec<_>>();
        let p4 = df(tspan[i] + h, &k4)
            .iter().map(|a| (h/6.)*a).collect::<Vec<_>>();
        // constructs the vector for a given timestep
        for k in 0..dim {
            y[i].push(y_prev[k] + p1[k] + p2[k] + p3[k] + p4[k]);
        }
        y_prev = y[i].clone();
    }
    y
}
