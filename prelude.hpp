#ifndef PRELUDE_HPP
#define PRELUDE_HPP

#include <array>
#include <cassert>
#include <deque>
#include <iostream>
#include <string>
#include <utility>
#include <vector>

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

struct Updatable {
    virtual void update() noexcept(false) = 0;
};

class Cell: public Updatable {
    Int* ptr;
    Int  initial;
    bool read;
    bool mutated;
    bool used;

public:
    constexpr explicit Cell(Int* ptr) noexcept
        : ptr(ptr), initial(*ptr), read(false), mutated(false), used(false) {}
    
    constexpr Int& get() noexcept(false) {
        if (mutated) {
            std::cout << "attempted to read mutated variable" << std::endl;
            throw nullptr;
        }

        read = true;
        return *ptr;
    }

    void update() noexcept(false) {
        if (*ptr != initial) {
            if (used) {
                std::cout << "mutation of used variable" << std::endl;
                throw nullptr;
            }

            initial = *ptr;
            mutated = true;
        }
        if (read) {
            used = true;
            read = false;
        }
    }
};

class IndexedCell: public Updatable {
    std::vector<Cell> data;

public:
    explicit IndexedCell(std::vector<Cell>&& data)
        : data(std::move(data)) {}

    constexpr Int& get(std::size_t i) {
        return data[i].get();
    }

    void update() noexcept(false) {
        for (auto& cell: data) {
            cell.update();
        }
    }
};

class Cells: public Updatable {
    std::vector<Updatable*> cells;

public:
    explicit Cells(std::vector<Updatable*>&& cells)
        : cells(std::move(cells)) {}

    void update() noexcept(false) {
        for (auto cell: cells) cell->update();
    }
};

Cell make_cell(Int& obj) { return Cell(&obj); }

template <std::size_t N>
IndexedCell make_cell(Array<N>& obj) {
    std::vector<Cell> data;
    data.reserve(N);
    for (auto& x: obj) data.emplace_back(&x);
    return IndexedCell(std::move(data));
}

IndexedCell make_cell(List& obj) {
    std::vector<Cell> data;
    for (auto& x: obj) data.emplace_back(&x);
    return IndexedCell(std::move(data));
}

template <class... Args>
Cells make_cells(Args&... args) { return Cells({&args...}); }

#endif
