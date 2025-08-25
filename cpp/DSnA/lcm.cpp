#include <iostream>
using namespace std;

// lcm = (a * b) / gcd(a, b)
// to avoid overflow we divide first the multiply
// lcm = (a / gcd(a, b)) * b

long long gcd(long long numA, long long numB) {
  while (numB != 0) {
    long long temp = numB;
    numB = numA % numB;
    numA = temp;
  }

  return numA;
}

long long lcm(long long numA, long long numB) {
  return (numA / gcd(numA, numB)) * numB;
}

int main() {
  long long numA, numB;
  cin >> numA >> numB;

  cout << lcm(numA, numB) << endl;

  return 0;
}
