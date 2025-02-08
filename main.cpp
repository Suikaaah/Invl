#include "prelude.hpp"


int main() {
    Array<3> a = {1, 2, 3};
    Int m = 0;
    Int n = 1;


    auto a_ = make_cell(a);
    auto m_ = make_cell(m);
    auto n_ = make_cell(n);
    auto cells = make_cells(a_, m_, n_);

    swap(a_.get(0), m_.get()); cells.update();
    swap(a_.get(n_.get()), a_.get(2)); cells.update();


    print("a", a);
    print("m", m);
    print("n", n);
}
