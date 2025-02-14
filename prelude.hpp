#ifndef PRELUDE_HPP
#define PRELUDE_HPP

#include <cassert>
#include <array>
#include <deque>
#include <iostream>
#include <string>
#include <utility>
#include <ranges>
#include <algorithm>
#include <boost/multiprecision/cpp_int.hpp>

using Int = boost::multiprecision::cpp_int;
using List = std::deque<Int>;
template <std::size_t N>
using Array = std::array<Int, N>;

template <class> struct S;
template <> struct S<List> {
    using Nonsense = void*;
    static constexpr const char* name() noexcept { return "list"; }
};
template <std::size_t N> struct S<Array<N>> {
    using Nonsense = void*;
    static std::string name() { return "array<" + std::to_string(N) + ">"; }
};
template <> struct S<Int> {
    static constexpr const char* name() noexcept { return "int"; }
};

template <class T, typename S<T>::Nonsense = nullptr>
std::ostream& operator<<(std::ostream& os, const T& l) {
    const char* delim = "";
    os << '[';
    for (const auto& x : l) {
        os << std::exchange(delim, ", ") << x;
    }
    return os << ']';
}

template <class T>
const Int& index(const T& l, const Int& i) {
    return l[i.convert_to<std::size_t>()];
}

template <class T>
Int& index(T& l, const Int& i) {
    return l[i.convert_to<std::size_t>()];
}

template <class T> void print(const char* name, const T& target) {
    std::cout << name << ": " << S<T>::name() << " = " << target << '\n';
}

template <class T>
void swap(T&& l, T&& r) {
    std::swap(std::forward<T>(l), std::forward<T>(r));
}

template <class T>
bool is_valid_perm(const T& l) {
    T copied = l;
    std::sort(copied.begin(), copied.end());
    for (std::size_t i{}; i < copied.size(); ++i) {
        if (i != copied[i]) return false;
    }

    return true;
}

template <class T, class U>
void assert_valid_perm(const T& c, const U& p) {
    assert(("not a valid permutation", c.size() == p.size() && is_valid_perm(p)));
}

// inj iota(const int n, list dst)
//     local int i = 0
//         from i = 0
//         loop
//             local int x = i
//                 push_back(x, dst)
//             delocal int x = 0
//             i += 1
//         until i = n
//     delocal int i = n
// 
// inj indices(const list src, list dst)
//     local const int n = size(src)
//         call iota(n, dst)
//     delocal const int n = size(src)

void iota_fwd(const Int& n, List& dst) {
    {
        Int i = 0;
        assert(i == 0);
        while (!(i == n)) {
            {
                Int x = i;
                dst.push_back(x);
                x = 0;
                assert(x == 0);
            }
            i += 1;
            assert(!(i == 0));
        }
        assert(i == n);
    }
}

void iota_rev(const Int& n, List& dst) {
    {
        Int i = n;
        assert(i == n);
        while (!(i == 0)) {
            i -= 1;
            {
                Int x = 0;
                assert(x == 0);
                x = dst.back();
                dst.pop_back();
                assert(x == i);
            }
            assert(!(i == n));
        }
        assert(i == 0);
    }
}

void indices_fwd(const List& src, List& dst) {
    {
        const Int n = src.size();
        iota_fwd(n, dst);
        assert(n == src.size());
    }
}

void indices_rev(const List& src, List& dst) {
    {
        const Int n = src.size();
        iota_rev(n, dst);
        assert(n == src.size());
    }
}

#endif
