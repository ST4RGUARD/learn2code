#include <iostream>
#include <vector>
using namespace std;

vector<long long> cache;

long long fib(int num) {
    if (num <= 1) return num;
    if (cache[num] != -1) return cache[num];

    cache[num] = fib(num - 1) + fib(num - 2);
    return cache[num];
}

int main() {
    int num;
    cin >> num;

    cache.assign(num + 1, -1);

    cout << fib(num) << endl;
    return 0;
}
