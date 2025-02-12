#include "prelude.hpp"

void f_fwd(List& l, Array<5>& a);
void f_rev(List& l, Array<5>& a);

int main() {
    List chars = {25, 39};
    Array<5> table = {149, 221, 29, 92, 283};


    f_fwd(chars, table);


    print("chars", chars);
    print("table", table);
}

void f_fwd(List& l, Array<5>& a) {

    for (const auto [i, j]: std::views::zip(l, a)) {
        i ^= j;
    }

}

void f_rev(List& l, Array<5>& a) {

    for (const auto [i, j]: std::views::zip(l, a)) {
        i ^= j;
    }

}
