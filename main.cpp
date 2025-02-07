#include"prelude.hpp"

void test_fwd(Int& v0, Int& v1, Int& v2, Int& v3);
void test_rev(Int& v0, Int& v1, Int& v2, Int& v3);
void f_fwd(Int& x, Int& y);
void f_rev(Int& x, Int& y);
void g_fwd(Array<4>& a);
void g_rev(Array<4>& a);

int main() {
Array<4> a = {1, 2, 3, 4};

print("a", a);

g_fwd(a);

print("a", a);
}

void test_fwd(Int& v0, Int& v1, Int& v2, Int& v3) {
Int v0_copied = v0;
Int v1_copied = v1;
Int v2_copied = v2;
Int v3_copied = v3;
v0 = 0 * v0_copied + 0 * v1_copied + 1 * v2_copied + 0 * v3_copied;
v1 = 0 * v0_copied + 0 * v1_copied + 0 * v2_copied + 1 * v3_copied;
v2 = 1 * v0_copied + 0 * v1_copied + 0 * v2_copied + 0 * v3_copied;
v3 = 0 * v0_copied + 1 * v1_copied + 0 * v2_copied + 0 * v3_copied;
}

void test_rev(Int& v0, Int& v1, Int& v2, Int& v3) {
Int v0_copied = v0;
Int v1_copied = v1;
Int v2_copied = v2;
Int v3_copied = v3;
v0 = 0 * v0_copied + 0 * v1_copied + 1 * v2_copied + 0 * v3_copied;
v1 = 0 * v0_copied + 0 * v1_copied + 0 * v2_copied + 1 * v3_copied;
v2 = 1 * v0_copied + 0 * v1_copied + 0 * v2_copied + 0 * v3_copied;
v3 = 0 * v0_copied + 1 * v1_copied + 0 * v2_copied + 0 * v3_copied;
}

void f_fwd(Int& x, Int& y) {
x += y;
y ^= x;
}

void f_rev(Int& x, Int& y) {
y ^= x;
x -= y;
}

void g_fwd(Array<4>& a) {
{
Int i = 0;
assert(i == 0);

while (!(i == a.size() - 1)) {
f_fwd(a[i], a[i + 1]);
i += 1;
assert(!(i == 0));

}
assert(i == a.size() - 1);
}

test_fwd(a[0], a[1], a[2], a[3]);

{
Int i = a.size() - 1;
assert(i == a.size() - 1);

while (!(i == 0)) {
i -= 1;
f_rev(a[i], a[i + 1]);
assert(!(i == a.size() - 1));

}
assert(i == 0);
}
}

void g_rev(Array<4>& a) {
{
Int i = 0;
assert(i == 0);

while (!(i == a.size() - 1)) {
f_fwd(a[i], a[i + 1]);
i += 1;
assert(!(i == 0));

}
assert(i == a.size() - 1);
}

test_fwd(a[0], a[1], a[2], a[3]);

{
Int i = a.size() - 1;
assert(i == a.size() - 1);

while (!(i == 0)) {
i -= 1;
f_rev(a[i], a[i + 1]);
assert(!(i == a.size() - 1));

}
assert(i == 0);
}
}
