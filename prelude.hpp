#ifndef PRELUDE_HPP
#define PRELUDE_HPP

#include <array>
#include <cassert>
#include <deque>
#include <iostream>
#include <string>
#include <utility>

using Int = int;
using List = std::deque<Int>;
template <std::size_t N>
using Array = std::array<Int, N>;

template <class> struct S;
template <> struct S<List> {
    using Vp = void*;
    static constexpr const char* name() { return "list"; }
};
template <std::size_t N> struct S<Array<N>> {
    using Vp = void*;
    static std::string name() { return "array[" + std::to_string(N) + "]"; }
};
template <> struct S<Int> {
    static constexpr const char* name() { return "int"; }
};

template <class T, typename S<T>::Vp = nullptr>
std::ostream& operator<<(std::ostream& os, const T& l) {
    os << '[';
    const char* delim = "";
    for (const auto& x : l) {
        os << std::exchange(delim, ", ") << x;
    }
    return os << ']';
}

template <class T> void print(const char* name, const T& target) {
    std::cout << name << ": " << S<T>::name() << " = " << target << '\n';
}

template <class T> constexpr void swap(T&& l, T&& r) {
    std::swap(std::forward<T>(l), std::forward<T>(r));
}

#endif
