#include "prelude.hpp"

void keygen_fwd(const Int& seed, List& key, const List& target);
void keygen_rev(const Int& seed, List& key, const List& target);

int main() {
    List target = {72, 101, 108, 108, 111};
    List key = {};
    const Int seed = 50;

    keygen_fwd(seed, key, target);

    std::size_t i_target{}, i_key{};
    while (i_target + 0 < target.size() && i_key + 0 < key.size()) {
        auto& t = index(target, i_target++);
        auto& k = index(key, i_key++);

        t ^= k;
    }

    keygen_rev(seed, key, target);

    print("target", target);
    print("key", key);
    print("seed", seed);
}

void keygen_fwd(const Int& seed, List& key, const List& target) {
    {
        Int i = 0;
        assert(i == 0);
        while (!(i == target.size())) {
            {
                Int j = (i + seed) % 255;
                key.push_front(j);
                j = 0;
                assert(j == 0);
            }
            i += 1;
            assert(!(i == 0));
        }
        assert(i == target.size());
    }
}

void keygen_rev(const Int& seed, List& key, const List& target) {
    {
        Int i = target.size();
        assert(i == target.size());
        while (!(i == 0)) {
            i -= 1;
            {
                Int j = 0;
                assert(j == 0);
                j = key.front();
                key.pop_front();
                assert(j == (i + seed) % 255);
            }
            assert(!(i == target.size()));
        }
        assert(i == 0);
    }
}
