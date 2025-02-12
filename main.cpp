#include "prelude.hpp"

void s_x_fwd(const Int& v0, Int& v1);
void s_x_rev(const Int& v0, Int& v1);
void s_y_fwd(Int& v0, const Int& v1);
void s_y_rev(Int& v0, const Int& v1);
void id_2_fwd(const Int& v0, const Int& v1);
void id_2_rev(const Int& v0, const Int& v1);
void negate_fwd(Int& v0);
void negate_rev(Int& v0);
void id_1_fwd(const Int& v0);
void id_1_rev(const Int& v0);
void c_minus_x_fwd(const Int& v0, Int& v1);
void c_minus_x_rev(const Int& v0, Int& v1);

int main() {
    List l{};
    List r{};


    swap(l, r);


    print("l", l);
    print("r", r);
}

void s_x_fwd(const Int& v0, Int& v1) {
    Int c1 = v1;
    v1 = 0 * v0 + -1 * c1;
}

void s_x_rev(const Int& v0, Int& v1) {
    Int c1 = v1;
    v1 = 0 * v0 + -1 * c1;
}

void s_y_fwd(Int& v0, const Int& v1) {
    Int c0 = v0;
    v0 = -1 * c0 + 0 * v1;
}

void s_y_rev(Int& v0, const Int& v1) {
    Int c0 = v0;
    v0 = -1 * c0 + 0 * v1;
}

void id_2_fwd(const Int& v0, const Int& v1) {
}

void id_2_rev(const Int& v0, const Int& v1) {
}

void negate_fwd(Int& v0) {
    Int c0 = v0;
    v0 = -1 * c0;
}

void negate_rev(Int& v0) {
    Int c0 = v0;
    v0 = -1 * c0;
}

void id_1_fwd(const Int& v0) {
}

void id_1_rev(const Int& v0) {
}

void c_minus_x_fwd(const Int& v0, Int& v1) {
    Int c1 = v1;
    v1 = 1 * v0 + -1 * c1;
}

void c_minus_x_rev(const Int& v0, Int& v1) {
    Int c1 = v1;
    v1 = 1 * v0 + -1 * c1;
}
