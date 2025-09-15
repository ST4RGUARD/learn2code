#include <exception>
#include <string>
#include <vector>
#include <iostream>
#include <fstream>
using namespace::std;

vector<string> tokenize(string csvline, char separator) {
  vector<string> tokens;
  size_t start = 0, end;
  while ((end = csvline.find(separator, start)) != string::npos) {
    tokens.push_back(csvline.substr(start, end - start));
    start = end + 1;
  }
  tokens.push_back(csvline.substr(start));
  return tokens;
}

int main() {
  vector<string> tokens;
  string line;

  ifstream csvFile {"dataset.csv"};
  if (csvFile.is_open())
  {
    cout << "open" << endl;
    while (getline(csvFile, line)){
      tokens = tokenize(line, ',');
      if (tokens.size() != 5) {
        cout << "Error: Expected 3 tokens, got " << tokens.size() << endl;
        continue;
      }

      try {
        double price = stod(tokens[3]);
        double amount = stod(tokens[4]);

        cout << "price:" << price << "amt:" << amount << endl;
      
      } catch (exception& e) {
        cout << "bad float" << tokens[3];
        cout << "bad float" << tokens[4];
      }

      double price = stod(tokens[3]);
      double amount = stod(tokens[4]);

      cout << "price:" << price << "amt:" << amount << endl;

      for (string& t: tokens){
        cout << t << endl;
      }
    }
    csvFile.close();
  }
  else
  {
    cout << "not open" << endl; 
  }
  return 0;
}
