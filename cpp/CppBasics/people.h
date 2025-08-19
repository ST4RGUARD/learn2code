#pragma once
#ifndef PEOPLE_H_INCLUDED
#define PEOPLE_H_INCLUDED

class B;        //declaration of declaration

class PersonalData
{
private:    //redundant private whether declared here or not
  // all vars should be private
  short age;
  int *p;
public:
  PersonalData();       //constructor
  ~PersonalData();      //destructor
  void setAge(int value);
  short getAge() { return age; }
  friend void change_data(PersonalData &, int);
};

class AA
{
  public:
    void setSecretvalue(B &, int);
};
class B
{
    friend class AA;
    int secretValue;
  public:
    int getSecretValue() { return secretValue; }
};

#endif