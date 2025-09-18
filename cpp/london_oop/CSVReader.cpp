#include "CSVReader.h"
#include "OrderBookEntry.h"
#include <exception>
#include <fstream>
#include <iostream>
#include <vector>

using namespace ::std;

CSVReader::CSVReader() {}

vector<OrderBookEntry> CSVReader::readCSV(string csvFileName) {
  vector<OrderBookEntry> entries;
  ifstream csvFile{csvFileName};
  string line;
  if (!csvFile.is_open()) {
    throw runtime_error("Could not open file");
  }
  while (getline(csvFile, line)) {
    try {
      vector<string> tokens = tokenize(line, ',');
      OrderBookEntry obe = stringToOBE(tokens);
      entries.push_back(obe);
    } catch (const exception &e) {
    }
  }
  csvFile.close();
  cout << "Read " << entries.size() << " entries from file "
       << csvFileName << endl;
  return entries;
}

vector<string> CSVReader::tokenize(string line, char separator) {
  vector<string> tokens;
  signed int start, end;
  string token;
  start = line.find_first_not_of(separator, 0);
  do {
    end = line.find_first_of(separator, start);
    if (start == line.length() || start == end)
      break;
    if (end >= 0)
      token = line.substr(start, end - start);
    else
      token = line.substr(start, line.length() - start);
    tokens.push_back(token);
    start = end + 1;
  } while (end > 0);
  return tokens;
}

OrderBookEntry CSVReader::stringToOBE(vector<string> tokens) {
  double price, amount;
  if (tokens.size() != 5) {
    cout << "bad line " << endl;
    throw exception{};
  }

  try {
    price = stod(tokens[3]);
    amount = stod(tokens[4]);
  } catch (const exception &e) {
    cout << "bad line";
    throw;
  }

  OrderBookEntry obe{price, amount, tokens[0], tokens[1],
                     OrderBookEntry::stringToOrderBookType(tokens[2])};
  return obe;
}
