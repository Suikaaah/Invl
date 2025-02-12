#include "prelude.hpp"

void s_x_fwd(Int& v0, Int& v1);
void s_x_rev(Int& v0, Int& v1);
void s_y_fwd(Int& v0, Int& v1);
void s_y_rev(Int& v0, Int& v1);

int main() {




}

void s_x_fwd(const Int& v0, Int& v1) {
    Int c0 = v0;
    Int c1 = v1;
    v1 = 0 * c0 + -1 * c1;
}

void s_x_rev(const Int& v0, Int& v1) {
    Int c0 = v0;
    Int c1 = v1;
    v1 = 0 * c0 + -1 * c1;
}

void s_y_fwd(Int& v0, const Int& v1) {
    Int c0 = v0;
    Int c1 = v1;
    v0 = -1 * c0 + 0 * c1;
}

void s_y_rev(Int& v0, const Int& v1) {
    Int c0 = v0;
    Int c1 = v1;
    v0 = -1 * c0 + 0 * c1;
}
