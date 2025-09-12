#include <vector>
#include "OrderBookEntry.h"

class MerkelMain {
  public:
    MerkelMain();
    /** Call this to start the sim */
    void init();

  private:
    void computeOrders();
    void printMenu();
    void processChoice(int choice);
    void userChoice();
    void displayOutput();
    double computeAveragePrice();
    double computeHighPrice();
    double computeLowPrice();
    double computePriceSpread();
    
    vector<OrderBookEntry> orders;
};
