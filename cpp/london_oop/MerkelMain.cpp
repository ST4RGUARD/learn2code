#include "MerkelMain.h"
#include "CSVReader.h"
#include "OrderBook.h"
#include "OrderBookEntry.h"
#include <iostream>
#include <limits>
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
  wallet.insertCurrency("BTC", 10);
  userChoice();
}

void MerkelMain::printMenu() {
  cout << "1: Print help" << endl;
  cout << "2: Print exchange stats" << endl;
  cout << "3: Make an ask" << endl;
  cout << "4: Make a bid" << endl;
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
    enterAsk();
    break;
  case 4:
    enterBid();
    break;
  case 5:
    printWallet();
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

void MerkelMain::printMarketStats() {
  for (string const &p : orderBook.getKnownProducts()) {
    cout << "Product: " << p << endl;
    vector<OrderBookEntry> entries =
        orderBook.getOrders(OrderBookType::ask, p, currentTime);
    cout << "Asks seen: " << entries.size() << endl;
    cout << "Max ask: " << OrderBook::getHighPrice(entries) << endl;
    cout << "Min ask: " << OrderBook::getLowPrice(entries) << endl;
    vector<OrderBookEntry> entries24hr =
        orderBook.getOrders24hr(OrderBookType::ask, p, currentTime);
    cout << "24Hr High: " << OrderBook::get24hrHigh(entries24hr) << endl;
  }
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

void MerkelMain::enterAsk() {
  cout << "Make an ask - enter the amount ETH/BTC,200,0.5" << endl;
  string input;
  // cin.ignore(numeric_limits<streamsize>::max(),'\n');
  getline(cin, input);
  vector<string> tokens = CSVReader::tokenize(input, ',');

  if (tokens.size() != 3) {
    cout << "MerkelMain::enterAsk Bad input!: " << input << endl;
  } else {
    try {
      OrderBookEntry obe = CSVReader::stringToOBE(
          tokens[1], tokens[2], currentTime, tokens[0], OrderBookType::ask);
      if (wallet.canFulfillOrder(obe)) {
        cout << "Wallet looks good. " << endl;
        orderBook.insertOrder(obe);
      } else {
        cout << "Wallet has insufficient funds. " << endl;
      }
    } catch (const exception &e) {
      cout << "MerkelMain::enterAsk Bad input!: " << input << endl;
    }
    cout << "You typed: " << input << endl;
  }
}

void MerkelMain::enterBid() {
  cout << "Make an bid - enter the amount ETH/BTC,200,0.5" << endl;
  string input;
  // cin.ignore(numeric_limits<streamsize>::max(),'\n');
  getline(cin, input);
  vector<string> tokens = CSVReader::tokenize(input, ',');

  if (tokens.size() != 3) {
    cout << "MerkelMain::enterBid Bad input!: " << input << endl;
  } else {
    try {
      OrderBookEntry obe = CSVReader::stringToOBE(
          tokens[1], tokens[2], currentTime, tokens[0], OrderBookType::bid);
      if (wallet.canFulfillOrder(obe)) {
        cout << "Wallet looks good. " << endl;
        orderBook.insertOrder(obe);
      } else {
        cout << "Wallet has insufficient funds. " << endl;
      }
    } catch (const exception &e) {
      cout << "MerkelMain::enterBid Bad input!: " << input << endl;
    }
    cout << "You typed: " << input << endl;
  }
}

void MerkelMain::printWallet() {
  cout << wallet.toString() << endl;
}
