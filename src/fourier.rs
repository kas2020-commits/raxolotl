extern crate num;

use num::complex::Complex;
use std::f64::consts::PI;

const I: Complex<f64> = Complex { re: 0.0, im: 1.0 };

/// Compute the fourier transform of a vector. Allocates a vector on the heap
/// and returns the value.
pub fn fft
(input: &[Complex<f64>]) -> Vec<Complex<f64>> {
    fn fft_inner(
        buf_a: &mut [Complex<f64>],
        buf_b: &mut [Complex<f64>],
        n: usize,    // total length of the input array
        step: usize, // precalculated values for t
    ) {
        if step >= n {
            return;
        }

        fft_inner(buf_b, buf_a, n, step * 2);
        fft_inner(&mut buf_b[step..], &mut buf_a[step..], n, step * 2);
        // create a slice for each half of buf_a:
        let (left, right) = buf_a.split_at_mut(n / 2);

        for i in (0..n).step_by(step * 2) {
            let t = (-I * PI * (i as f64) / (n as f64)).exp() * buf_b[i + step];
            left[i / 2] = buf_b[i] + t;
            right[i / 2] = buf_b[i] - t;
        }
    }

    // round n (length) up to a power of 2:
    let n = input.len().next_power_of_two();
    // copy the input into a buffer:
    let mut buf_a = input.to_vec();
    // right pad with zeros to a power of two:
    buf_a.append(&mut vec![Complex { re: 0.0, im: 0.0 }; n - input.len()]);
    // alternate between buf_a and buf_b to avoid allocating a new vector each time:
    let mut buf_b = buf_a.clone();

    fft_inner(&mut buf_a, &mut buf_b, n, 1);
    buf_a
}

/// Computes the inverse fourier transform using the fft algorithm. Allocates a
/// vector on the heap and returns the value.
pub fn ifft
(input: &[Complex<f64>]) -> Vec<Complex<f64>> {
    fft(&input
        .into_iter()
        .map(|x| x.conj().unscale(input.len() as f64))
        .collect::<Vec<_>>()
    )
}
