#include "prelude.hpp"

void f_fwd(const Int& v0, const Int& v1);
void f_rev(const Int& v0, const Int& v1);

int main() {
    List l = {2, 5, 9, 3, 4};
    const Int i = 0;
    const Int j = 1;


    f_fwd(i, j);


    print("l", l);
    print("i", i);
    print("j", j);
}

void f_fwd(const Int& v0, const Int& v1) {
}

void f_rev(const Int& v0, const Int& v1) {
}
