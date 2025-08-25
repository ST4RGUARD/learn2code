#include <iostream>
using namespace std;

// gcd(a, b) = gcd(b, a % b)

void gcd(int numA, int numB) {
  while (numB != 0) {
    int temp = numB;
    numB = numA % numB;
    numA = temp;
  }

  cout << numA << endl;
}

int main() {
  int numA, numB;
  cin >> numA >> numB;

  gcd(numA, numB);

  return 0;
}
