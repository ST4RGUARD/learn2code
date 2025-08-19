#include <iostream>
using namespace std;

int main() {
    int count;
    cin >> count;
    long long* arr = new long long[count];
    for (int i = 0; i < count; ++i) {
        cin >> arr[i];
    }

    long long max1 = -1, max2 = -1;
    for (int i = 0; i < count; ++i) {
        if (arr[i] > max1) {
            max2 = max1;
            max1 = arr[i];
        } else if (arr[i] > max2) {
            max2 = arr[i];
        }
    }

    cout << max1 * max2;

    delete[] arr;
    return 0;
}
