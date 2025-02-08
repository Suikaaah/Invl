#include "prelude.hpp"

void mirror_fwd(Int& v0, Int& v1, Int& v2, Int& v3);
void mirror_rev(Int& v0, Int& v1, Int& v2, Int& v3);
void negate_fwd(Int& v0);
void negate_rev(Int& v0);
void a_fwd(Int& v0, Int& v1);
void a_rev(Int& v0, Int& v1);

int main() {
    Int w = 3;
    Int x = 4;
    Int y = 5;
    Int z = 6;

    w += z;
    x -= y;

    mirror_fwd(w, x, y, z);

    x += y;
    w -= z;

    print("w", w);
    print("x", x);
    print("y", y);
    print("z", z);
}

void mirror_fwd(Int& v0, Int& v1, Int& v2, Int& v3) {
    Int c0 = v0;
    Int c1 = v1;
    Int c2 = v2;
    Int c3 = v3;
    v0 = 0 * c0 + 0 * c1 + 0 * c2 + 1 * c3;
    v1 = 0 * c0 + 0 * c1 + 1 * c2 + 0 * c3;
    v2 = 0 * c0 + 1 * c1 + 0 * c2 + 0 * c3;
    v3 = 1 * c0 + 0 * c1 + 0 * c2 + 0 * c3;
}

void mirror_rev(Int& v0, Int& v1, Int& v2, Int& v3) {
    Int c0 = v0;
    Int c1 = v1;
    Int c2 = v2;
    Int c3 = v3;
    v0 = 0 * c0 + 0 * c1 + 0 * c2 + 1 * c3;
    v1 = 0 * c0 + 0 * c1 + 1 * c2 + 0 * c3;
    v2 = 0 * c0 + 1 * c1 + 0 * c2 + 0 * c3;
    v3 = 1 * c0 + 0 * c1 + 0 * c2 + 0 * c3;
}

void negate_fwd(Int& v0) {
    Int c0 = v0;
    v0 = -1 * c0;
}

void negate_rev(Int& v0) {
    Int c0 = v0;
    v0 = -1 * c0;
}

void a_fwd(Int& v0, Int& v1) {
    Int c0 = v0;
    Int c1 = v1;
    v0 = 5 * c0 + -6 * c1;
    v1 = 4 * c0 + -5 * c1;
}

void a_rev(Int& v0, Int& v1) {
    Int c0 = v0;
    Int c1 = v1;
    v0 = 5 * c0 + -6 * c1;
    v1 = 4 * c0 + -5 * c1;
}
