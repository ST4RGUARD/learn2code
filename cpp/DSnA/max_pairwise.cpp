#include <iostream>
using namespace std;

int main() {
    int count;
    cin >> count;
    long long* arr = new long long [count];
    for (int i = 0; i < count; ++i) {
        cin >> arr[i];
    }

    // Bubble sort
    for (int i = 0; i < count - 1; i++) {
        for (int j = 0; j < count - i - 1; j++) {
            if (arr[j] > arr[j + 1]) {
                int temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
    
    cout << arr[count - 1] * arr[count - 2];
    delete[] arr;
    return 0;
}
