#include <iostream>
#include <stdexcept>
#include <string>
#include <vector>
using namespace std;

enum class OrderBookType{bid, ask};
vector<string> timestamps;
vector<string> products;
vector<OrderBookType> orderTypes;
vector<double> prices;
vector<double> amounts;

ostream& operator<<(ostream& os, OrderBookType t) {
    switch (t) {
        case OrderBookType::bid:  return os << "bid";
        case OrderBookType::ask: return os << "ask";
    }
    return os << "Unknown OrderBookType";
}

void print_menu()
{
  cout << "1: Print help" << endl;
  cout << "2: Print exchange stats" << endl;
  cout << "3: Place an ask" << endl;
  cout << "4: Place a bid" << endl;
  cout << "5: Print wallet" << endl;
  cout << "6: Continue" << endl;
  cout << "7: Exit" << endl;
}

void process_choice(int choice)
{
  switch (choice) {
    case 1:
      cout << "++ Help menu ++" << endl;
      print_menu();
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

void user_choice()
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
      process_choice(choice);
    } catch (invalid_argument&) {
      cout << "Invalid input. Please enter a number between 1 and 7." << endl;
    }
  }
}

void mock_data()
{
  timestamps.push_back("2025/09/10 17:35:24.654642");
  products.push_back("BTC/USDT");
  orderTypes.push_back(OrderBookType::bid);
  prices.push_back(5000.1);
  amounts.push_back(5.1);

  cout << "    " << "\t   Time   \t" << "    " << "\tProducts\t" << "Order Type" << "\t" << "Price" << "\t" << "Amount" << endl;
  cout << timestamps[0] << "\t" << products[0] << "\t" << "    " << orderTypes[0] << "\t\t" << prices[0] << "   " << amounts[0] << endl;
}

int main()
{
  print_menu();
  user_choice();
  mock_data();
}
