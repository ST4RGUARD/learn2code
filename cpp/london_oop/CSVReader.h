#pragma once

#include "OrderBookEntry.h"
#include <vector>

class CSVReader {
public:
  CSVReader();

  static vector<OrderBookEntry> readCSV(string csvFileName);

private:
  static vector<string> tokenize(string line, char separator);
  static OrderBookEntry stringToOBE(vector<string> strings);
};
