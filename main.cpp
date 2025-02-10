#include "prelude.hpp"

void f_fwd(List& l, List& r, Int& temp);
void f_rev(List& l, List& r, Int& temp);

int main() {
    List l = {7, 8, 9};
    List r = {1, 2, 3, 4};
    Int temp{};


    auto l_ = make_cell(l);
    auto r_ = make_cell(r);
    auto temp_ = make_cell(temp);
    Cells cells(l_, r_, temp_);

    f_fwd((*l_), (*r_), (*temp_));
    cells.update();


    print("l", l);
    print("r", r);
    print("temp", temp);
}

void f_fwd(List& l, List& r, Int& temp) {
    temp += l.size();

    auto l_ = make_cell(l);
    auto r_ = make_cell(r);
    auto temp_ = make_cell(temp);
    Cells cells(l_, r_, temp_);

    {
        Cell x_;
        cells.push(x_);
        for (Int i = 0; i < (*temp_); ++i) {
            x_ = make_cell(i); (*x_);
            cells.update();

        }
        cells.pop();
    }
    swap((*l_), (*r_));
    cells.update();

    temp -= l.size();
}

void f_rev(List& l, List& r, Int& temp) {
    temp += l.size();

    auto l_ = make_cell(l);
    auto r_ = make_cell(r);
    auto temp_ = make_cell(temp);
    Cells cells(l_, r_, temp_);

    {
        Cell x_;
        cells.push(x_);
        for (Int i = 0; i < (*temp_); ++i) {
            x_ = make_cell(i); (*x_);
            cells.update();

        }
        cells.pop();
    }
    swap((*l_), (*r_));
    cells.update();

    temp -= l.size();
}
