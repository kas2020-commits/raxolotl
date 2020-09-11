
pub fn composite_midpoint
(n: u32, f: impl Fn(f64) -> f64, a: f64, b: f64) -> f64 {
    let h = (b - a) / (n as f64 + 2f64);
    let mut sum = f(a) + f(b);
    for i in -1..(n as i32 / 2) {
        sum += f(a + (((2f64 * i as f64) + 1f64) * h));
    }
    2f64 * h * sum
}

pub fn composite_trapezoid
(n: u32, f: impl Fn(f64) -> f64, a: f64, b: f64) -> f64 {
    let h = (b - a) / (n as f64);
    let mut sum = f(a) + f(b);
    for i in 1..n {
        sum += 2f64 * f(a + (i as f64 * h));
    }
    (h / 2f64) * sum
}

pub fn composite_simpson
(n: u32, f: impl Fn(f64) -> f64, a: f64, b: f64) -> f64 {
    let h = (b - a) / n as f64;
    let mut sum = f(a) + f(b);
    for i in 1..n/2 {
        sum += 2f64 * f(a + (i as f64 * 2f64 * h));
    }
    for j in 1..(n/2 + 1) {
        sum += 4f64 * f(a + (((j as f64 * 2f64) - 1f64) * h));
    }
    (h / 3f64) * sum
}

