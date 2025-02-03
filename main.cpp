#include "prelude.hpp"

void crypt_fwd(std::array<int, 3> &target, std::array<int, 3> &key,
               std::array<int, 3> &temp);
void crypt_rev(std::array<int, 3> &target, std::array<int, 3> &key,
               std::array<int, 3> &temp);

int main() {
    std::array<int, 3> target = {217, 157, 96};
    std::array<int, 3> key = {220, 159, 99};
    std::array<int, 3> temp = {0, 0, 0};

    print("target", target);
    print("key", key);
    print("temp", temp);

    crypt_fwd(target, key, temp);

    print("temp", temp);
    print("key", key);
    print("target", target);
}

void crypt_fwd(std::array<int, 3> &target, std::array<int, 3> &key,
               std::array<int, 3> &temp) {
    {
        int index = 0;
        assert(index == 0);

        while (!(index == 3)) {
            temp[index] += target[index] ^ key[index];
            index += 1;
            assert(!(index == 0));
        }
        assert(index == 3);
    }

    std::swap(temp, target);

    {
        int index = 3;
        assert(index == 3);

        while (!(index == 0)) {
            index -= 1;
            temp[index] -= target[index] ^ key[index];
            assert(!(index == 3));
        }
        assert(index == 0);
    }
}

void crypt_rev(std::array<int, 3> &target, std::array<int, 3> &key,
               std::array<int, 3> &temp) {
    {
        int index = 0;
        assert(index == 0);

        while (!(index == 3)) {
            temp[index] += target[index] ^ key[index];
            index += 1;
            assert(!(index == 0));
        }
        assert(index == 3);
    }

    std::swap(temp, target);

    {
        int index = 3;
        assert(index == 3);

        while (!(index == 0)) {
            index -= 1;
            temp[index] -= target[index] ^ key[index];
            assert(!(index == 3));
        }
        assert(index == 0);
    }
}
