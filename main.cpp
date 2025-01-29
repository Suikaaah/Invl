#include "prelude.hpp"

void fib_fwd(int &x1, int &x2, int &n);
void fib_rev(int &x1, int &x2, int &n);

int main() {
    int x1{};
    int x2{};
    int n{};

    x1 += 5;
    x2 += 8;
    fib_rev(x1, x2, n);

    std::cout << "x1: " << x1 << '\n';
    std::cout << "x2: " << x2 << '\n';
    std::cout << "n: " << n << '\n';
}

void fib_fwd(int &x1, int &x2, int &n) {
    if (n == 0) {
        x1 += 1;
        x2 += 1;
        assert(x1 == x2);
    } else {
        n -= 1;
        fib_fwd(x1, x2, n);
        x1 += x2;
        std::swap(x1, x2);
        assert(!(x1 == x2));
    }
}

void fib_rev(int &x1, int &x2, int &n) {
    if (x1 == x2) {
        x2 -= 1;
        x1 -= 1;
        assert(n == 0);
    } else {
        std::swap(x1, x2);
        x1 -= x2;
        fib_rev(x1, x2, n);
        n += 1;
        assert(!(n == 0));
    }
}
