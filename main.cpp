#include "prelude.hpp"

void swap_front_and_back_fwd(List& l, List& perm);
void swap_front_and_back_rev(List& l, List& perm);
void swap_front_and_back_impl_fwd(List& l, const List& perm);
void swap_front_and_back_impl_rev(List& l, const List& perm);

int main() {
    List l = {3, 9};
    List temp{};


    swap_front_and_back_fwd(l, temp);


    print("l", l);
    print("temp", temp);
}

void swap_front_and_back_fwd(List& l, List& perm) {
    indices_fwd(l, perm);
    swap(index(perm, 1), index(perm, perm.size() - 1));

    swap_front_and_back_impl_fwd(l, perm);

    swap(index(perm, 1), index(perm, perm.size() - 1));
    indices_rev(l, perm);
}

void swap_front_and_back_rev(List& l, List& perm) {
    indices_fwd(l, perm);
    swap(index(perm, 1), index(perm, perm.size() - 1));

    swap_front_and_back_impl_fwd(l, perm);

    swap(index(perm, 1), index(perm, perm.size() - 1));
    indices_rev(l, perm);
}

void swap_front_and_back_impl_fwd(List& l, const List& perm) {

    std::size_t i_l{}, i_perm{};
    assert_valid_perm(l, perm);
    while (i_l + 1 < l.size() && i_perm + 1 < perm.size()) {
        auto& x = index(l, perm[i_l++]);
        auto& y = index(l, perm[i_l++]);
        auto& p = index(perm, i_perm++);
        auto& q = index(perm, i_perm++);

        if (p == 0 || q == 0) {
            swap(x, y);
        } else {
        }
    }

}

void swap_front_and_back_impl_rev(List& l, const List& perm) {

    std::size_t i_l{}, i_perm{};
    assert_valid_perm(l, perm);
    while (i_l + 1 < l.size() && i_perm + 1 < perm.size()) {
        auto& x = index(l, perm[i_l++]);
        auto& y = index(l, perm[i_l++]);
        auto& p = index(perm, i_perm++);
        auto& q = index(perm, i_perm++);

        if (p == 0 || q == 0) {
            swap(x, y);
        } else {
        }
    }

}
