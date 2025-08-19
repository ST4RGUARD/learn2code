#include <iostream>
#include <vector>
#include <algorithm>
#include <cstdlib>

int main(int argc, char* argv[]) {
    std::vector<int> numbers;

    for (int i = 1; i < argc; i++) {
        numbers.push_back(std::stoi(argv[i]));
    }

    std::sort(numbers.begin(), numbers.end());

    std::cout << "Sorted numbers: ";
    for (int n : numbers) {
        std::cout << n << " ";
    }
    std::cout << std::endl;

    return 0;
}

