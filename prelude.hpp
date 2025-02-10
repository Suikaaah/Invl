#ifndef PRELUDE_HPP
#define PRELUDE_HPP

#include <array>
#include <cassert>
#include <deque>
#include <iostream>
#include <string>
#include <utility>
#include <vector>
#include <stack>

using Int = int;
using List = std::deque<Int>;
template <std::size_t N>
using Array = std::array<Int, N>;

template <class> struct S;
template <> struct S<List> {
    using Vp = void*;
    static constexpr const char* name() noexcept { return "list"; }
};
template <std::size_t N> struct S<Array<N>> {
    using Vp = void*;
    static std::string name() { return "array<" + std::to_string(N) + ">"; }
};
template <> struct S<Int> {
    static constexpr const char* name() noexcept { return "int"; }
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

void swap(Int& l, Int& r) noexcept {
    std::swap(l, r);
}

template <std::size_t N>
void swap(Array<N>& l, Array<N>& r) {
    std::swap(l, r);
}

// element-wise swap (invalidates existing pointers for popped elements)
void swap(List& l, List& r) {
    auto& smaller = l.size() < r.size() ? l : r;
    auto& larger = l.size() < r.size() ? r : l;
    const auto size_smaller = smaller.size();
    const auto size_larger = larger.size();
    for (std::size_t i{}; i < size_smaller; ++i) {
        swap(smaller[i], larger[i]);
    }
    std::stack<Int> stack;
    for (std::size_t i{}; i < size_larger - size_smaller; ++i) {
        stack.push(larger.back());
        larger.pop_back();
    }
    while (!stack.empty()) {
        smaller.push_back(stack.top());
        stack.pop();
    }
}

struct Updatable { virtual void update() = 0; };

class Cell: public Updatable {
    Int *ptr, initial;
    bool read, mutated_delayed, read_delayed;

public:
    Cell() {}
    explicit Cell(Int& obj)
        : ptr(&obj), initial(obj), read(false), mutated_delayed(false), read_delayed(false) {}
    
    Int& operator*() {
        assert(("attempted to read mutated variable", !is_mutated()));
        read = true;
        return *ptr;
    }

    void update() final {
        const auto first_mutation = is_mutated() && !mutated_delayed;

        if (first_mutation) {
            assert(("mutation of used variable", !read_delayed));
            mutated_delayed = true;
        }

        read_delayed |= read;
    }

    bool is_mutated() const { return *ptr != initial; }
};

template <class T>
class IndexedCell: public Updatable {
    T* container;
    std::vector<Cell> elements;
    std::size_t size;
    bool container_read, element_read, size_changed_delayed, container_read_delayed, element_read_delayed;

public:
    explicit IndexedCell(T& obj)
        : container(&obj), size(obj.size()),
          container_read(false), element_read(false),
          size_changed_delayed(false), container_read_delayed(false), element_read_delayed(false) {
        elements.reserve(size);
        for (auto& x: obj) elements.emplace_back(x);
    }

    bool size_changed() const { return container->size() != size; }
    void verify_size() const { assert(("size changed", !size_changed())); }
    
    T& operator*() {
        verify_size();
        for (auto& x: elements) *x;
        container_read = true;
        return *container;
    }

    Int& operator[](std::size_t i) {
        verify_size();
        element_read = true;
        return *elements[i];
    }

    void update() final {
        const auto first_size_change = size_changed() && !size_changed_delayed;

        if (first_size_change) {
            assert(("mutation of used variable", !container_read_delayed && !element_read_delayed));
            size_changed_delayed = true;
        }
        
        if (!size_changed()) for (auto& x: elements) x.update();

        container_read_delayed |= container_read;
        element_read_delayed |= element_read;
    }
};

class Cells: public Updatable {
    std::vector<Updatable*> cells;

public:
    template <class... Args>
    explicit Cells(Args&... args)
        : cells{&args...} {}

    void update() final {
        for (auto cell: cells) cell->update();
    }

    template <class T>
    void push(T& obj) { cells.emplace_back(&obj); }

    void pop() { cells.pop_back(); }
};

Cell make_cell(Int& obj) {
    return Cell(obj);
}

template <std::size_t N>
IndexedCell<Array<N>> make_cell(Array<N>& obj) {
    return IndexedCell<Array<N>>(obj);
}

IndexedCell<List> make_cell(List& obj) {
    return IndexedCell<List>(obj);
}

#endif
