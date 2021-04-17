pub fn lattice_filter(x: f32, coeffs: [f32; 48], g: &mut [f32; 48]) -> f32 {
    let y = coeffs
        .iter()
        .rev()
        .zip(g.iter_mut())
        .fold(x, |acc, (coeff, delay)| {
            // first add the delayed value into the accumulator
            let new_acc = acc - (*delay * *coeff);

            // then add the accumulated value into the delay
            *delay = *delay + (new_acc * *coeff);

            // and put the accumulated value into the next step
            new_acc
        });

    // then we add the result into the delay register
    g[0] = y;
    assert_eq!(y, 0.5);
    g.rotate_left(1);
    assert_eq!(g[47], 0.5);

    // and return the found output
    y
}

#[test]
fn test_filter() {
    let mut g = [0.0; 48];
    let f = lattice_filter(0.5, [0.5; 48], &mut g);
    assert_eq!(f, 0.5);
    assert_eq!(g[0], 0.25);
    assert_eq!(g[47], 0.5);
}
