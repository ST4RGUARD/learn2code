#include "OrderBookEntry.h"
#include "OrderBook.h"
#include <vector>

class MerkelMain {
public:
  MerkelMain();
  /** Call this to start the sim */
  void init();

private:
  void computeOrders();
  void printMenu();
  void printMarketStats();
  void processChoice(int choice);
  void userChoice();
  void gotoNextTimeframe();
  void displayOutput();
  double computeAveragePrice();
  double computeHighPrice();
  double computeLowPrice();
  double computePriceSpread();

  string currentTime;

  OrderBook orderBook{"dataset.csv"};
};
