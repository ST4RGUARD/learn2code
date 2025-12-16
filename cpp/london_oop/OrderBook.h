#pragma once
#include "CSVReader.h"
#include "OrderBookEntry.h"
#include <string>
#include <vector>

using namespace std;

class OrderBook {
public:
  /** construct reading a CSV file */
  OrderBook(string filename);
  /** return vector of all known products */
  vector<string> getKnownProducts();
  /** return vector of orders according to the sent filters */
  vector<OrderBookEntry> getOrders(OrderBookType type,
                                   string product,
                                   string timestamp);

private:
  vector<OrderBookEntry> orders;

};
