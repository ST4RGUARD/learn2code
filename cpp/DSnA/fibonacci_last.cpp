#include <iostream>
#include <string>
#include <vector>
using namespace std;

vector<long long> cache;

long long fib(int num) {
  if (num == 0)
    return 0;
  if (num == 1)
    return 1;

  int first = 0, second = 1, next = 0;
  for (int i = 2; i <= num; i++) {
    next = (first + second) % 10;
    first = second;
    second = next;
  }
  return second;
}

void last(int num) {
  string num_str = to_string(num);
  char last_num = num_str.back();

  cout << last_num << endl;
}

int main() {
  int num;
  cin >> num;

  last(fib(num));

  return 0;
}
