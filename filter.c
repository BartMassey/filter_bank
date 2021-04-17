#include<assert.h>

float lattice_filter(float x, float coeffs[48], float g[48]) {
    float acc = x;
    for (int i = 0; i < 48; i++) {
        float coeff = coeffs[i];
        float *delay = &g[47 - i];

        // first add the delayed value into the accumulator
        float new_acc = acc - *delay * coeff;

        // then add the accumulated value into the delay
        *delay += new_acc * coeff;

        // and put the accumulated value into the next step
        acc = new_acc;
    }

    // then we add the result into the delay register
    // and rotate left
    for (int i = 1; i < 48; i++) {
        g[i - 1] = g[i];
    }
    g[47] = acc;

    // and return the found output
    return acc;
}

float g[48];
float coeffs[48];

int main() {
    for (int i = 0; i < 48; i++) {
        coeffs[i] = 0.5;
    }
    float f = lattice_filter(0.5, coeffs, g);
    assert(f == 0.5);
    assert(g[0] == 0.25);
    assert(g[47] == 0.5);
}
