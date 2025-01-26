#ifndef PRELUDE_HPP
#define PRELUDE_HPP

#include <array>
#include <cassert>
#include <deque>
#include <iostream>
template <class T> struct IsListOrArray {
    static constexpr bool value = false;
};
template <> struct IsListOrArray<std::deque<int>> {
    static constexpr bool value = true;
};
template <std::size_t N> struct IsListOrArray<std::array<int, N>> {
    static constexpr bool value = true;
};
template <class T, std::enable_if_t<IsListOrArray<T>::value, void *> = nullptr>
std::ostream &operator<<(std::ostream &os, const T &l) {
    os << '[';
    const char *delim = "";
    for (auto x : l) {
        os << std::exchange(delim, ", ") << x;
    }
    return os << ']';
}

#endif