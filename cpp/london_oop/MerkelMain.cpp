#include "MerkelMain.h"
#include "OrderBookEntry.h"
#include <iostream>
#include <ostream>
#include <string>
using namespace std;

ostream &operator<<(ostream &os, OrderBookType t) {
  switch (t) {
  case OrderBookType::bid:
    return os << "bid";
  case OrderBookType::ask:
    return os << "ask";
  case OrderBookType::unknown:
    return os << "unknown";
  }
  return os << "Unknown OrderBookType";
}

MerkelMain::MerkelMain() {}
void MerkelMain::init() {
  currentTime = orderBook.getEarliestTime();
  userChoice();
  displayOutput();
}

void MerkelMain::printMenu() {
  cout << "1: Print help" << endl;
  cout << "2: Print exchange stats" << endl;
  cout << "3: Place an ask" << endl;
  cout << "4: Place a bid" << endl;
  cout << "5: Print wallet" << endl;
  cout << "6: Continue" << endl;
  cout << "7: Exit" << endl;
  cout << "============================" << endl;
  cout << "Current time is: " << currentTime << endl;
}

void MerkelMain::processChoice(int choice) {
  switch (choice) {
  case 1:
    cout << "++ Help menu ++" << endl;
    break;
  case 2:
    MerkelMain::printMarketStats();
    break;
  case 3:
    cout << "Place ankask not implemented yet." << endl;
    break;
  case 4:
    cout << "Place a bid not implemented yet." << endl;
    break;
  case 5:
    cout << "Wallet not implemented yet." << endl;
    break;
  case 6:
    gotoNextTimeframe();
    break;
  default:
    cout << "Invalid choice. Please type in 1-7." << endl;
    break;
  }
}

void MerkelMain::gotoNextTimeframe() {
  cout << "Going to next time frame" << endl;
  currentTime = orderBook.getNextTime(currentTime);
}

void MerkelMain::userChoice() {
  while (true) {
    string input;
    printMenu();
    cout << "Type in 1-7" << endl;
    getline(cin, input);
    try {
      int choice = stoi(input);
      if (choice == 7) {
        cout << "Exiting..." << endl;
        break;
      }
      MerkelMain::processChoice(choice);
    } catch (invalid_argument &) {
      cout << "Invalid input. Please enter a number between 1 and 7." << endl;
    }
  }
}

double MerkelMain::computeAveragePrice() {
  double avg = 0.0;
  // for (int i = 0; i < orders.size(); ++i) {
  //   avg += orders[i].price;
  // }
  // return avg / orders.size();
}

double MerkelMain::computeLowPrice() {
  double low = 0.0;
  // for (int i = 0; i < orders.size(); ++i) {
  //   if (i == 0) {
  //     low = orders[i].price;
  //   }
  //
  //   if (orders[i].price < low) {
  //     low = orders[i].price;
  //   }
  // }
  return low;
}

double MerkelMain::computeHighPrice() {
  double high = 0.0;
  // for (int i = 0; i < orders.size(); ++i) {
  //   if (orders[i].price > high) {
  //     high = orders[i].price;
  //   }
  // }
  return high;
}

double MerkelMain::computePriceSpread() {
  return computeHighPrice() - computeLowPrice();
}

void MerkelMain::printMarketStats() {
  for (string const &p : orderBook.getKnownProducts()) {
    cout << "Product: " << p << endl;
    vector<OrderBookEntry> entries = orderBook.getOrders(
        OrderBookType::ask, p, "2020/03/17 17:01:24.884492");
    cout << "Asks seen: " << entries.size() << endl;
    cout << "Max ask: " << OrderBook::getHighPrice(entries) << endl;
    cout << "Min ask: " << OrderBook::getLowPrice(entries) << endl;
  }
  // cout << "orders size: " << orders.size() << endl;
  // unsigned int bids = 0;
  // unsigned int asks = 0;
  // for (OrderBookEntry obe : orders) {
  //   if (obe.orderType == OrderBookType::bid) {
  //     bids++;
  //   } else if (obe.orderType == OrderBookType::ask) {
  //     asks++;
  //   }
  // }
  // cout << "Bids: " << bids << endl;
  // cout << "Asks: " << asks << endl;
}

void MerkelMain::displayOutput() {
  cout << "++ Prices ++" << endl;
  cout << "Avg: " << computeAveragePrice() << endl;
  cout << "Low: " << computeLowPrice() << endl;
  cout << "High: " << computeHighPrice() << endl;
  cout << "Spread: " << computePriceSpread() << endl;
}
