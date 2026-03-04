#pragma once

#include "OrderBookEntry.h"
#include <vector>

class CSVReader {
public:
  CSVReader();

  static vector<OrderBookEntry> readCSV(string csvFileName);
  static vector<string> tokenize(string line, char separator);
  static OrderBookEntry stringToOBE(string price, string amount,
                                    string timestamp, string product,
                                    OrderBookType orderType);

private:
  static OrderBookEntry stringToOBE(vector<string> strings);
};
