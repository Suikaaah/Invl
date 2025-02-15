#include "prelude.hpp"

void setup_fwd(List& l, List& b);
void setup_rev(List& l, List& b);

int main() {
    List l = {0, 1, 2, 3, 4, 5, 6, 7};
    List b{};

    setup_fwd(l, b);

    std::size_t i_l{}, i_b{};
    while (i_l + 1 < l.size() && i_b + 0 < b.size()) {
        auto& x = index(l, i_l++);
        auto& y = index(l, i_l++);
        auto& p = index(b, i_b++);

        if (p) {
            swap(x, y);
        }
    }

    setup_rev(l, b);

    print("l", l);
    print("b", b);
}

void setup_fwd(List& l, List& b) {
    {
        const Int s = l.size();
        {
            Int i = 0;
            assert(i == 0);
            while (!(i == s)) {
                if (i < 2) {
                    b.push_back(1);
                    assert(i < 2);
                } else {
                    b.push_back(0);
                    assert(!(i < 2));
                }
                i += 1;
                assert(!(i == 0));
            }
            assert(i == s);
        }
        swap(index(l, 1), index(l, 2));
        swap(index(l, 1), index(l, s - 1));
        swap(index(l, 3), index(l, s - 2));
        assert(s == l.size());
    }
}

void setup_rev(List& l, List& b) {
    {
        const Int s = l.size();
        swap(index(l, 3), index(l, s - 2));
        swap(index(l, 1), index(l, s - 1));
        swap(index(l, 1), index(l, 2));
        {
            Int i = s;
            assert(i == s);
            while (!(i == 0)) {
                i -= 1;
                if (i < 2) {
                    assert(1 == b.back());
                    b.pop_back();
                    assert(i < 2);
                } else {
                    assert(0 == b.back());
                    b.pop_back();
                    assert(!(i < 2));
                }
                assert(!(i == s));
            }
            assert(i == 0);
        }
        assert(s == l.size());
    }
}
