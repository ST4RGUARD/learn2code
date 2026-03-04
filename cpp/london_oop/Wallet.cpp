#include "Wallet.h"
#include "CSVReader.h"
#include <string>
#include <iostream>

Wallet::Wallet() {}

void Wallet::insertCurrency(string type, double amount) {
  double balance;
  if (amount < 0)
  {
    throw exception{};
  }
  if (currencies.count(type) == 0)
  {
    balance = 0;
  }
  else {
    balance = currencies[type];
  }
  balance += amount;
  currencies[type] = balance;
}

bool Wallet::removeCurrency(string type, double amount) {
  if (amount < 0)
  {
    return false; 
  }
  if (currencies.count(type) == 0)
  {
    return false;
  }
  else {
    if (containsCurrency(type, amount))
    {
      currencies[type] -= amount;
      return true;
    }
    else
      return false;       
  }
}

bool Wallet::containsCurrency(string type, double amount)
{
  if (currencies.count(type) == 0) 
   return false;
  else
    return currencies[type] >= amount;
}
string Wallet::toString()
{
  string s;
  for (pair<string,double> pair : currencies)
  {
    string currency = pair.first;
    double amount = pair.second;
    s += currency + " : " + to_string(amount) + "\n";
  }
  return s;
}

bool Wallet::canFulfillOrder(OrderBookEntry order)
{
  vector<string> currs = CSVReader::tokenize(order.product, '/');
  // ask
  if (order.orderType == OrderBookType::ask)
  {
    double amount = order.amount;
    string currency = currs[0];
    cout << "Wallet::canFulfillOrder " << currency << " : " << amount << endl;
    return containsCurrency(currency, amount);
  }
  // bid
  if (order.orderType == OrderBookType::bid)
  {
    double amount = order.amount * order.price;
    string currency = currs[1];
    cout << "Wallet::canFulfillOrder " << currency << " : " << amount << endl;
    return containsCurrency(currency, amount);
  }

  return false;
}
