#include "OrderBookEntry.h"
#include "OrderBook.h"
#include "Wallet.h"
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
  void enterBid();
  void enterAsk();
  void printWallet();
  void processChoice(int choice);
  void userChoice();
  void gotoNextTimeframe();

  string currentTime;

  OrderBook orderBook{"dataset.csv"};
  Wallet wallet;
};
