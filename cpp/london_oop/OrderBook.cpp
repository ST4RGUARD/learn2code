#include "OrderBook.h"
#include "CSVReader.h"
#include "OrderBookEntry.h"
#include <chrono>
#include <iomanip>
#include <map>
#include <sstream>
#include <vector>
using namespace std;

OrderBook::OrderBook(string filename) { orders = CSVReader::readCSV(filename); }

vector<string> OrderBook::getKnownProducts() {
  vector<string> products;
  map<string, bool> prodMap;
  for (OrderBookEntry &e : orders) {
    prodMap[e.product] = true;
  }
  for (auto const &e : prodMap) {
    products.push_back(e.first);
  }
  return products;
}

string OrderBook::getEarliestTime() {
  if (orders.empty())
    return "";
  return orders[0].timestamp;
}

string OrderBook::getNextTime(string timestamp) {
  string next_timestamp = "";
  for (OrderBookEntry &e : orders) {
    if (e.timestamp > timestamp) {
      next_timestamp = e.timestamp;
      break;
    }
  }
  if (next_timestamp == "") {
    next_timestamp = orders[0].timestamp;
  }
  return next_timestamp;
}

vector<OrderBookEntry> OrderBook::getOrders(OrderBookType type, string product,
                                            string timestamp) {
  vector<OrderBookEntry> orders_sub;
  for (OrderBookEntry &e : orders) {
    if (e.orderType == type && e.product == product &&
        e.timestamp == timestamp) {
      orders_sub.push_back(e);
    }
  }
  return orders_sub;
}

vector<OrderBookEntry>
OrderBook::getOrders24hr(OrderBookType type, string product, string timestamp) {
  vector<OrderBookEntry> orders_sub;
  auto end_time = parseTimestamp(timestamp);
  auto start_time = end_time - std::chrono::hours(24);
  for (OrderBookEntry &e : orders) {
    if (e.orderType == type && e.product == product) {
      auto entry_time = parseTimestamp(e.timestamp);
      if (entry_time >= start_time && entry_time <= end_time) {
        orders_sub.push_back(e);
      }
    }
  }
  return orders_sub;
}

std::chrono::system_clock::time_point
OrderBook::parseTimestamp(const string &timestamp) {
  std::tm tm = {};
  double seconds = 0.0;
  std::istringstream ss(timestamp);
  ss >> std::get_time(&tm, "%Y/%m/%d %H:%M:%S");
  size_t dot_pos = timestamp.find('.');
  if (dot_pos != string::npos) {
    seconds = std::stod(timestamp.substr(dot_pos));
  }
  auto tp = std::chrono::system_clock::from_time_t(std::mktime(&tm));
  tp += std::chrono::duration_cast<std::chrono::system_clock::duration>(
      std::chrono::duration<double>(seconds));
  return tp;
}

double OrderBook::getHighPrice(vector<OrderBookEntry> &orders) {
  double max = orders[0].price;
  for (OrderBookEntry &e : orders) {
    if (e.price > max)
      max = e.price;
  }
  return max;
}

double OrderBook::getLowPrice(vector<OrderBookEntry> &orders) {
  double min = orders[0].price;
  for (OrderBookEntry &e : orders) {
    if (e.price < min)
      min = e.price;
  }
  return min;
}

// Returns the highest price from a vector of OrderBookEntry objects within a
// 24-hour period. If the input vector is empty, returns 0. Uses dataset and
// assumes the vector contains orders filtered for the desired product, type and
// 24-hour window
double OrderBook::get24hrHigh(vector<OrderBookEntry> &orders) {
  if (orders.empty())
    return 0.0;
  double max = orders[0].price;
  for (OrderBookEntry &e : orders) {
    if (e.price > max)
      max = e.price;
  }
  return max;
}

void OrderBook::insertOrder(OrderBookEntry &order) {
  orders.push_back(order);
  sort(orders.begin(), orders.end(), OrderBookEntry::compareByTimestamp );
}
