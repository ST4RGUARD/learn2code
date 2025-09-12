#include <iostream>
#include <ostream>
#include <string>
#include <vector>
#include "MerkelMain.h"
using namespace std;

ostream& operator<<(ostream& os, OrderBookType t) {
    switch (t) {
        case OrderBookType::bid:  return os << "bid";
        case OrderBookType::ask: return os << "ask";
    }
    return os << "Unknown OrderBookType";
}

MerkelMain::MerkelMain()
{

}
void MerkelMain::init()
{
  printMenu();
  userChoice();
  computeOrders();
  displayOutput();
}

void MerkelMain::computeOrders()
{
  orders.push_back( OrderBookEntry{.255,
                    7.0,
                    "2025/09/10 17:35:24.654642",
                    "BTC/USDT",
                    OrderBookType::bid} );

  orders.push_back( OrderBookEntry{.252,
                    7.5,
                    "2025/09/10 15:35:24.654642",
                    "BTC/USDT",
                    OrderBookType::ask} );


  for (int i = 0; i < orders.size(); ++i)
  {
    cout << "The amount is: " << orders[i].amount << endl;
    cout << "The price is: " << orders[i].price << endl;
    cout << "The Order Type is: " << orders[i].orderType << endl << endl;
  }
}

void MerkelMain::printMenu()
{
  cout << "1: Print help" << endl;
  cout << "2: Print exchange stats" << endl;
  cout << "3: Place an ask" << endl;
  cout << "4: Place a bid" << endl;
  cout << "5: Print wallet" << endl;
  cout << "6: Continue" << endl;
  cout << "7: Exit" << endl;
}

void MerkelMain::processChoice(int choice)
{
  switch (choice) {
    case 1:
      cout << "++ Help menu ++" << endl;
      MerkelMain::printMenu();
      break;
    case 2:
      cout << "Exchange stats not implemented yet." << endl;
      break;
    case 3:
      cout << "Place an ask not implemented yet." << endl;
      break;
    case 4:
      cout << "Place a bid not implemented yet." << endl;
      break;
    case 5:
      cout << "Wallet not implemented yet." << endl;
      break;
    case 6:
      cout << "Continuing..." << endl;
      break;
    default:
      cout << "Invalid choice. Please type in 1-7." << endl;
      break;
  }
}

void MerkelMain::userChoice()
{
  while (true) {
    string input; 
    cout << "Type in 1-7" << endl;
    getline(cin, input);
    try {
      int choice = stoi(input);
      if (choice == 7) {
        cout << "Exiting..." << endl;
        break;
    }
      MerkelMain::processChoice(choice);
    } catch (invalid_argument&) {
      cout << "Invalid input. Please enter a number between 1 and 7." << endl;
    }
  }
}

double MerkelMain::computeAveragePrice()
{
  double avg = 0.0;
  for (int i = 0; i < orders.size(); ++i)
  {
    avg += orders[i].price;
  }
  return avg / orders.size();
}

double MerkelMain::computeLowPrice()
{
  double low = 0.0;
  for (int i = 0; i < orders.size(); ++i)
  {
    if (i == 0)
    {
      low = orders[i].price;
    }

    if (orders[i].price < low)
    {
      low = orders[i].price;
    } 
  }
  return low;
}

double MerkelMain::computeHighPrice()
{
  double high = 0.0;
  for (int i = 0; i < orders.size(); ++i)
  {
    if (orders[i].price > high)
    {
      high = orders[i].price;
    } 
  }
  return high;
}

double MerkelMain::computePriceSpread()
{
  return computeHighPrice() - computeLowPrice();
}

void MerkelMain::displayOutput()
{
  cout << "++ Prices ++" << endl;
  cout << "Avg: "    << computeAveragePrice()<< endl;
  cout << "Low: "    << computeLowPrice()    << endl;
  cout << "High: "   << computeHighPrice()   << endl;
  cout << "Spread: " << computePriceSpread() << endl;
}
