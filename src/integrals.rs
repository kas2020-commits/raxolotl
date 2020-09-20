
type Num = f64;

/// Simplest composite method out there. Computes the midpoint at every step
/// then adds that to the sum.
/// Local error: O(h^2)
pub fn composite_midpoint
(n: u32, f: impl Fn(Num) -> Num, a: Num, b: Num) -> Num {
    let h = (b - a) / (n as Num + 2.0);
    let mut sum = f(a) + f(b);
    let _n = (n >> 1) as i32;
    for i in -1.._n {
        sum += f(a + (((2 * i) + 1) as Num * h));
    }
    2.0 * h * sum
}

/// Order higher than midpoint method. Essentially midpoint with double the
/// number of steps.
/// Local error: O(h^3)
pub fn composite_trapezoid
(n: u32, f: impl Fn(Num) -> Num, a: Num, b: Num) -> Num {
    let h = (b - a) / (n as Num);
    let mut sum = f(a) + f(b);
    for i in 1..n {
        sum += 2.0 * f(a + (i as Num * h));
    }
    (h / 2.0) * sum
}

/// Higher order than composite trapezoid method, Simpson is not a terrible
/// method. For very large values of n, parallelism might make this very fast
/// as compared to previous models
/// Local error: O(h^4)
pub fn composite_simpson
(n: u32, f: impl Fn(Num) -> Num, a: Num, b: Num) -> Num {
    let h = (b - a) / n as Num;
    let mut sum = f(a) + f(b);
    let _n = n >> 1;
    for i in 1.._n {
        sum += 2.0 * f(a + (i as Num * 2.0 * h));
    }
    for j in 1..(n/2 + 1) {
        sum += 4.0 * f(a + (((j as Num * 2.0) - 1.0) * h));
    }
    (h / 3.0) * sum
}

