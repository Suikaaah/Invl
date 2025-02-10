#include "prelude.hpp"

void c_minus_x_fwd(Int& v0, Int& v1);
void c_minus_x_rev(Int& v0, Int& v1);
void f_fwd(List& l);
void f_rev(List& l);

int main() {
    List l = {3, 2};


    auto l_ = make_cell(l);
    Cells cells(l_);

    f_fwd((*l_));
    cells.update();


    print("l", l);
}

void c_minus_x_fwd(Int& v0, Int& v1) {
    Int c0 = v0;
    Int c1 = v1;
    v0 = 1 * c0 + 0 * c1;
    v1 = 1 * c0 + -1 * c1;
}

void c_minus_x_rev(Int& v0, Int& v1) {
    Int c0 = v0;
    Int c1 = v1;
    v0 = 1 * c0 + 0 * c1;
    v1 = 1 * c0 + -1 * c1;
}

void f_fwd(List& l) {

    auto l_ = make_cell(l);
    Cells cells(l_);

    {
        Cell x_;
        cells.push(x_);
        for (Int i = 0; i < l.size(); ++i) {
            x_ = make_cell(i);

            l_[(*x_)] ^= 3254908;
            cells.update();
        }
        cells.pop();
    }

}

void f_rev(List& l) {

    auto l_ = make_cell(l);
    Cells cells(l_);

    {
        Cell x_;
        cells.push(x_);
        for (Int i = 0; i < l.size(); ++i) {
            x_ = make_cell(i);

            l_[(*x_)] ^= 3254908;
            cells.update();
        }
        cells.pop();
    }

}
