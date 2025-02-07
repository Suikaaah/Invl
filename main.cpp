#include "prelude.hpp"

void test_fwd(Int& v0, Int& v1, Int& v2, Int& v3);
void test_rev(Int& v0, Int& v1, Int& v2, Int& v3);
void f_fwd(Int& x, Int& y);
void f_rev(Int& x, Int& y);
void g_fwd(Array<4>& a);
void g_rev(Array<4>& a);

int main() {
    Array<4> a = {-12, 23, -13, 7};

    print("a", a);

    g_fwd(a);

    print("a", a);
}

void test_fwd(Int& v0, Int& v1, Int& v2, Int& v3) {
    Int c0 = v0;
    Int c1 = v1;
    Int c2 = v2;
    Int c3 = v3;
    v0 = 0 * c0 + 0 * c1 + 1 * c2 + 0 * c3;
    v1 = 0 * c0 + 0 * c1 + 0 * c2 + 1 * c3;
    v2 = 1 * c0 + 0 * c1 + 0 * c2 + 0 * c3;
    v3 = 0 * c0 + 1 * c1 + 0 * c2 + 0 * c3;
}

void test_rev(Int& v0, Int& v1, Int& v2, Int& v3) {
    Int c0 = v0;
    Int c1 = v1;
    Int c2 = v2;
    Int c3 = v3;
    v0 = 0 * c0 + 0 * c1 + 1 * c2 + 0 * c3;
    v1 = 0 * c0 + 0 * c1 + 0 * c2 + 1 * c3;
    v2 = 1 * c0 + 0 * c1 + 0 * c2 + 0 * c3;
    v3 = 0 * c0 + 1 * c1 + 0 * c2 + 0 * c3;
}

void f_fwd(Int& x, Int& y) {
    x += y;
    y ^= x;
}

void f_rev(Int& x, Int& y) {
    y ^= x;
    x -= y;
}

void g_fwd(Array<4>& a) {
    {
        Int i = 0;
        assert(i == 0);
        while (!(i == a.size() - 1)) {
            f_fwd(a[i], a[i + 1]);
            i += 1;
            assert(!(i == 0));
        }
        assert(i == a.size() - 1);
    }

    test_fwd(a[0], a[1], a[2], a[3]);

    {
        Int i = a.size() - 1;
        assert(i == a.size() - 1);
        while (!(i == 0)) {
            i -= 1;
            f_rev(a[i], a[i + 1]);
            assert(!(i == a.size() - 1));
        }
        assert(i == 0);
    }
}

void g_rev(Array<4>& a) {
    {
        Int i = 0;
        assert(i == 0);
        while (!(i == a.size() - 1)) {
            f_fwd(a[i], a[i + 1]);
            i += 1;
            assert(!(i == 0));
        }
        assert(i == a.size() - 1);
    }

    test_fwd(a[0], a[1], a[2], a[3]);

    {
        Int i = a.size() - 1;
        assert(i == a.size() - 1);
        while (!(i == 0)) {
            i -= 1;
            f_rev(a[i], a[i + 1]);
            assert(!(i == a.size() - 1));
        }
        assert(i == 0);
    }
}
