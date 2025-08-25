#include <iostream>
using namespace std;

int fib(int num) {
  if (num <= 1) {
    return num;
  } else {
    return fib(num - 1) + fib(num - 2);
  }
}

int main() {
    int num;
    cin >> num;
    
    cout << fib(num) << endl;

  return 0;
}
