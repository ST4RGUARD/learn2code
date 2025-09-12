#include "OrderBookEntry.h"
    
OrderBookEntry::OrderBookEntry(
      double _price,
      double _amount,
      string _timestamp,
      string _product,
      OrderBookType _orderType)

    : price(_price), amount(_amount), timestamp(_timestamp), product(_product), orderType(_orderType)
{

}
