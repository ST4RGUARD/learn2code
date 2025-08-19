#include "people.h"
#include <iostream>

using namespace std;

void PersonalData::setAge(int age ) //55 goes here
{
  if (age < 0)
    this->age = 0; // assign to short age in private method
  else
    this->age = age;
}

PersonalData::PersonalData()
{
  static int i = 0;
  cout << "constructor has been invoked " << ++i << endl;
  age = 10;

  p = new int[5];
}

PersonalData::~PersonalData()
{
  static int j = 0;
  cout << "destructor has been invoked " << ++j << endl;
  age = 10;

  delete [] p;
}

void AA::setSecretvalue(B & obj, int value)
{
  obj.secretValue = value;
}