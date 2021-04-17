pub fn lattice_filter(x: f32, coeffs: [f32; 48], g: &mut [f32; 48]) -> f32 {
    let mut acc = x;
    let pairs = coeffs
        .iter()
        .rev()
        .cloned()
        .zip(g.iter_mut());
    for (coeff, delay) in pairs {
        let new_acc;
        if cfg!(feature = "fma") {
            // first add the delayed value into the accumulator
            new_acc = f32::mul_add(-coeff, *delay, acc);

            // then add the accumulated value into the delay
            *delay = f32::mul_add(coeff, new_acc, *delay);
        } else {
            // first add the delayed value into the accumulator
            new_acc = acc - *delay * coeff;

            // then add the accumulated value into the delay
            *delay += new_acc * coeff;
        }

        // and put the accumulated value into the next step
        acc = new_acc;
    }

    // then we add the result into the delay register
    g[0] = acc;
    g.rotate_left(1);

    // and return the found output
    acc
}

#[test]
fn test_filter() {
    let mut g = [0.0; 48];
    let f = lattice_filter(0.5, [0.5; 48], &mut g);
    assert_eq!(f, 0.5);
    assert_eq!(g[0], 0.25);
    assert_eq!(g[47], 0.5);
}
