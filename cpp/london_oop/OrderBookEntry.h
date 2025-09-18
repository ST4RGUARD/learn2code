#pragma once

#include <string>
using namespace std;

enum class OrderBookType { bid, ask, unknown};

class OrderBookEntry {
public:
  OrderBookEntry(double _price, double _amount, string _timestamp,
                 string _product, OrderBookType _orderType);

  static OrderBookType stringToOrderBookType(string s);

  double price;
  double amount;
  string timestamp;
  string product;
  OrderBookType orderType;
};
