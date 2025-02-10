#include "prelude.hpp"

void f_fwd(List& l, Int& m, Int& n);
void f_rev(List& l, Int& m, Int& n);

int main() {
    List l{};
    Array<3> a = {1, 2, 3};


    auto l_ = make_cell(l);
    auto a_ = make_cell(a);
    Cells cells(l_, a_);

    f_fwd((*l_), a_[0], a_[1]);
    cells.update();
    a_[2] ^= 255;
    cells.update();


    print("l", l);
    print("a", a);
}

void f_fwd(List& l, Int& m, Int& n) {
    if (0 < m) {
        l.push_front(m);
        m = 0;
        assert(m <= 0);
    } else {
        swap(m, n);
        assert(!(m <= 0));
    }

    auto l_ = make_cell(l);
    auto m_ = make_cell(m);
    auto n_ = make_cell(n);
    Cells cells(l_, m_, n_);

    swap((*m_), (*n_));
    cells.update();

    if (m <= 0) {
        assert(m == 0);
        m = l.front();
        l.pop_front();
        assert(0 < m);
    } else {
        swap(m, n);
        assert(!(0 < m));
    }
}

void f_rev(List& l, Int& m, Int& n) {
    if (0 < m) {
        l.push_front(m);
        m = 0;
        assert(m <= 0);
    } else {
        swap(m, n);
        assert(!(m <= 0));
    }

    auto l_ = make_cell(l);
    auto m_ = make_cell(m);
    auto n_ = make_cell(n);
    Cells cells(l_, m_, n_);

    swap((*m_), (*n_));
    cells.update();

    if (m <= 0) {
        assert(m == 0);
        m = l.front();
        l.pop_front();
        assert(0 < m);
    } else {
        swap(m, n);
        assert(!(0 < m));
    }
}
