#include "prelude.hpp"

void f_fwd(List& l, List& r, Int& min);
void f_rev(List& l, List& r, Int& min);

int main() {
    List l = {1, 2, 3, 4};
    List r = {9, 8, 7};
    Int min{};


    auto l_ = make_cell(l);
    auto r_ = make_cell(r);
    auto min_ = make_cell(min);
    Cells cells(l_, r_, min_);

    f_fwd((*l_), (*r_), (*min_));
    cells.update();


    print("l", l);
    print("r", r);
    print("min", min);
}

void f_fwd(List& l, List& r, Int& min) {
    if (l.size() < r.size()) {
        min += l.size();
        assert(l.size() < r.size());
    } else {
        min += r.size();
        assert(!(l.size() < r.size()));
    }

    auto l_ = make_cell(l);
    auto r_ = make_cell(r);
    auto min_ = make_cell(min);
    Cells cells(l_, r_, min_);

    swap((*l_), (*r_));
    cells.update();

    if (l.size() < r.size()) {
        min -= l.size();
        assert(l.size() < r.size());
    } else {
        min -= r.size();
        assert(!(l.size() < r.size()));
    }
}

void f_rev(List& l, List& r, Int& min) {
    if (l.size() < r.size()) {
        min += l.size();
        assert(l.size() < r.size());
    } else {
        min += r.size();
        assert(!(l.size() < r.size()));
    }

    auto l_ = make_cell(l);
    auto r_ = make_cell(r);
    auto min_ = make_cell(min);
    Cells cells(l_, r_, min_);

    swap((*l_), (*r_));
    cells.update();

    if (l.size() < r.size()) {
        min -= l.size();
        assert(l.size() < r.size());
    } else {
        min -= r.size();
        assert(!(l.size() < r.size()));
    }
}
