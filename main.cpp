#include "prelude.hpp"

void f_fwd(List& l, Array<5>& a, Int& count);
void f_rev(List& l, Array<5>& a, Int& count);

int main() {
    List chars = {25, 39};
    Array<5> table = {149, 221, 29, 92, 283};
    Int count = 0;


    auto chars_ = make_cell(chars);
    auto table_ = make_cell(table);
    auto count_ = make_cell(count);
    Cells cells(chars_, table_, count_);

    f_fwd((*chars_), (*table_), (*count_));
    cells.update();


    print("chars", chars);
    print("table", table);
    print("count", count);
}

void f_fwd(List& l, Array<5>& a, Int& count) {
    if (l.size() < a.size()) {
        count += l.size();
        assert(l.size() < a.size());
    } else {
        count += a.size();
        assert(!(l.size() < a.size()));
    }

    auto l_ = make_cell(l);
    auto a_ = make_cell(a);
    auto count_ = make_cell(count);
    Cells cells(l_, a_, count_);

    if (2 < (*count_)) {
        l_[0] ^= a_[0];
        cells.update();
    } else {
        {
            Cell i_;
            cells.push(i_);
            for (Int i = 0; i < (*count_); ++i) {
                i_ = make_cell(i); (*i_);
                cells.update();

                l_[(*i_)] ^= a_[(*i_)];
                cells.update();
            }
            cells.pop();
        }
    }

    if (l.size() < a.size()) {
        count -= l.size();
        assert(l.size() < a.size());
    } else {
        count -= a.size();
        assert(!(l.size() < a.size()));
    }
}

void f_rev(List& l, Array<5>& a, Int& count) {
    if (l.size() < a.size()) {
        count += l.size();
        assert(l.size() < a.size());
    } else {
        count += a.size();
        assert(!(l.size() < a.size()));
    }

    auto l_ = make_cell(l);
    auto a_ = make_cell(a);
    auto count_ = make_cell(count);
    Cells cells(l_, a_, count_);

    if (2 < (*count_)) {
        l_[0] ^= a_[0];
        cells.update();
    } else {
        {
            Cell i_;
            cells.push(i_);
            for (Int i = 0; i < (*count_); ++i) {
                i_ = make_cell(i); (*i_);
                cells.update();

                l_[(*i_)] ^= a_[(*i_)];
                cells.update();
            }
            cells.pop();
        }
    }

    if (l.size() < a.size()) {
        count -= l.size();
        assert(l.size() < a.size());
    } else {
        count -= a.size();
        assert(!(l.size() < a.size()));
    }
}
