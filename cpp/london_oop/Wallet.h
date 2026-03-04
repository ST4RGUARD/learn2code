#include "OrderBookEntry.h"
#include <string>
#include <map>
using namespace std;

class Wallet {
  public:
    Wallet();
    /** insert currency to the wallet */
    void insertCurrency(string type, double amount);
    /** remove currency from the wallet */
    bool removeCurrency(string type, double amount);
    /** check if the wallet contains this much currency or more */
    bool containsCurrency(string type, double amount);
    /** chech if wallet can cope with this bid or ask */ 
    bool canFulfillOrder(OrderBookEntry order);
    /** generate a string representation of the wallet */
    string toString();

  private:
    map<string,double> currencies;
};
