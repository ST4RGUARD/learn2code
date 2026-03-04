#pragma once
#include "CSVReader.h"
#include "OrderBookEntry.h"
#include <chrono>
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
  vector<OrderBookEntry> getOrders(OrderBookType type, string product,
                                   string timestamp);
  vector<OrderBookEntry> getOrders24hr(OrderBookType type, string product,
                                       string timestamp);
  string getEarliestTime();
  string getNextTime(string timestamp);

  void insertOrder(OrderBookEntry& order);
  static double getHighPrice(vector<OrderBookEntry> &orders);
  static double getLowPrice(vector<OrderBookEntry> &orders);
  static double get24hrHigh(vector<OrderBookEntry> &orders);

private:
  vector<OrderBookEntry> orders;
  static std::chrono::system_clock::time_point
  parseTimestamp(const string &timestamp);
};
