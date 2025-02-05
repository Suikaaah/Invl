#ifndef PRELUDE_HPP
#define PRELUDE_HPP

#include <array>
#include <cassert>
#include <deque>
#include <iostream>
#include <string>

using Int = int;
using List = std::deque<int>;
template <std::size_t N>
using Array = std::array<int, N>;

template <class> struct S;

template <> struct S<List> {
    using Vp = void *;
    static const char *name() { return "list"; }
};

template <std::size_t N> struct S<Array<N>> {
    using Vp = void *;
    static std::string name() { return "array[" + std::to_string(N) + "]"; }
};

template <> struct S<Int> {
    static const char *name() { return "int"; }
};

template <class T, S<T>::Vp = nullptr>
std::ostream &operator<<(std::ostream &os, const T &l) {
    os << '[';
    const char *delim = "";
    for (auto &x : l) {
        os << std::exchange(delim, ", ") << x;
    }
    return os << ']';
}

template <class T> void print(const char *name, const T &target) {
    std::cout << name << ": " << S<T>::name() << " = " << target << '\n';
}

#endif
