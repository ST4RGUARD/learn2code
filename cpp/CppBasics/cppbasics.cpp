#include <iostream>
#include <string>
#include <time.h>
#include "people.h"
//#include "ourlib.h"                   //  include custom header 

#define PI 3.14

using namespace std;

// hodgepodge of fundamental c++ concepts and examples

enum dayOfWeek{M,TU=3,W,TH,F,SA,SU};

// extern int a; //external var means we don't reserve mem for it - 
// used in this code or other cpp file but def extern in hdr 

// function declarations
int power(int, int);
void user_options();
void select_shape(int);
void calc_shape_area(double, double, int);
bool is_valid(string);
string getDay(dayOfWeek);
void swap(int &, int &);
void lottery(int, int);
void test_construc_destruc();
void change_data(PersonalData, int);

// class template
// in .h class temp like func change all types to T - 
// template<typename T> 
// class - Add<T>::func(T x) construc & for deconstructor
// function template
template<typename T, typename T2>
T add(T var1, T var2)
{
  return var1 + var2;
}

struct personal_data {
  string name;
  string surname;
  string number;
  short age;
} mine,yours;

// overloading functions
double power(double, int);
// inline function
double add(double a, double b) { return a + b; }

void change_value(int a)
{
  // makes a copy of a so not modifying a within main
  a = 10;
  // also local variable is destroyed after return of func
}

int fib(int n) {
	if (n < 3) {
		return 1;
	}
	else 
	{
		return(fib(n - 2) + fib(n - 1));
	}
}

// namespaces
namespace A
{
  int ns = 20;
}

namespace mySpace
{
  class MyNewLine
  {
    string text;
  public:
    MyNewLine(string text = "\n\n\n") { this->text = text; }
    string toString()
    {
      return text;
    }
  };
  MyNewLine endl;
}

int main() {
	short  t1;    // -32768 to 32767, 2bytes
	float  t2;    // 4bytes up to 38 zeros
	double t3;    // 8bytes up to 308zeros more precision
	int A = 4;
	char cont, t4;    // character
	string t5 = "blah blah";
	string t6 = " gah gah";
	string combine = t5 + t6;
	const string NAMEOFGAME = "the due";
	bool   t7 = true;
	int a,b,num, choice,t;
  int ref1 = 10;
  int ref2 = 20;
  int p_var = 5;
  int *p_v1;      // asterisk here is used to declare this var a pointer
  // pointers
  // int * const p_v1 is constant ptr can't be re-assigned so has to be initialized to address
  // ordinary var that store addresses of variables
  p_v1 = &p_var;
  cout << "p_var value:  "<< p_var << " address: " << &p_var << endl << endl;

  // asterisk here is used to Retrieve value from indicated (pointed) area in memory (address)
  cout << "p_v1  pts_to: " << p_v1 << " p_v1 value: " << *p_v1 << "  address: " << &p_v1 << endl ;

  // so can change value of p_var by changing pointer
  *p_v1 = 20;
  // or can point to diff var by getting address of that var
  p_v1 = &ref2;
  
  cout << "p_var: " << p_var << endl;
  cout << "*p: " << *p_v1 << endl;


  // reference var - must be initialized when created
  // can change value but can't change var associated with ref var
  // ref var must be same type as assoc var
  string &cmb = combine;

  // just changing x & y in swap not ref1 & ref2 here
  swap(ref1, ref2);

  cout << "ref1: " << ref1 << endl;
  cout << "ref2: " << ref2 << endl;

  // enum example
  dayOfWeek d = TU;
  cout << d << endl;
  cout << dayOfWeek(3) << endl;
  cout << getDay(TU) << endl;
  a = 5;
  
  B objB;
  AA objA;

  objA.setSecretvalue(objB,600);
  cout << objB.getSecretValue() << endl;

  change_value(a);
  cout << " a is " << a << endl ;
	// bitwise practice
	cout << (10 & 2) << endl;
	/*
	1 0 1 0
	0 0 1 0
	- - - -
	0 0 1 0 
	*/
	cout << (10 | 2) << endl;
	/*
	1 0 1 0
	0 0 1 0
	- - - -
	1 0 1 0
	*/
	cout << (10 ^ 2) << endl;
	/*
	1 0 1 0
	0 0 1 0
	- - - -
	1 0 0 0
	*/
	cout << (~10) << endl;
	/*
	1 0 1 0
	0 0 1 0
	*/
	
  // reference address of var
  cout << A << endl;
	cout << &A << endl;
	cout << "this is test\n";
	cin >> b;
	cout << "b=" << b << endl;
  //

	// ternary conditional
	a = 5;
	b = 6;
	string message = (a > b) ? "a>b" : "a<=b";

	// can get cryptic but doable
	cout << ((a > b ? a : b) + 10) << endl;


  // function
	cout << "enter fib : ";
	cin >> num;
	cout << "fib is : " << fib(num) << endl;
  //

  // inline function
  cout << add(4, 5);
  //

  // multi arrays
	/*
	int y[3][4];
	y[1][1];
     1   2  3  4
	1[] [] [] []
	2[] [x][] []
	3[] [] [] []

	*/

	// loops
	string tmp;
	cout << "Enter number: " << endl;
	cin >> tmp;

	cout << "number is : ";
	for (int i = 0; i < tmp.length(); i++)
	{
		if (tmp[i] >= 48 && tmp[i] <= 57)
			cout << tmp[i];
		else
			continue;
	}

	cout << endl;
	cout << endl;

	for (int i = 1; i <= 10; i++) // amt of rows, len of columns
	{
		if (i == 5)
			continue;
	
		for (int j = 1; j <= 10; j++) { // amt of columns, len of rows
			if (j == 5)
				break;
			cout.width(4);
			cout << i * j << " ";
		}
		cout << endl;
	}

	while (a < 10)
	{
		cout << " blah " << endl;
		a++;
	}

	do
	{
		cout << "i execute at least once" << endl;
	} while (a < 10);
  //

  cout << power(2.8, 3) << endl;

  // calculate shape\

  do
  {
    system("cls");                                            // clear screen
    user_options();
    
    while (!(cin >> choice))
    {
      is_valid("incorrect choice");    // ignore all but 1
    }

    select_shape(choice);
    do
    {
      cout << "continue Y/N ?" << endl;
      cin >> cont;                                            // do not have to clear with char
      cin.ignore(numeric_limits<streamsize>::max(), '\n');    // ignore all but 1

    } while (cont != 'y' && cont != 'Y' && cont != 'n' && cont != 'N');

  } while (cont == 'y' || cont == 'Y');

  // dynamic allocation of memory
  // need free space AFTER compiling
  // new - memory belongs only to our program not free
  
  int *new_ptr = new int;
  int new_var = 5555;
  *new_ptr = 50;

  cout << *new_ptr << endl;

  delete new_ptr;                               // delete - release memory reserved by new to be used later
  new_ptr = &new_var;                           // after delete we can store address of new var np

  new_ptr = NULL;
  //*new_ptr = 60;                              // should not do this after delete..memory is not ours
  if (new_ptr != NULL)                          // here we set a condition if we haven't deleted/freed the memory we can re-assign it
  {
    *new_ptr = 60;
    cout << *new_ptr << endl;

  }

  cout << new_ptr << endl;

  // str to char
  string text = "blahblah";
  const char * text2 = text.c_str();
  cout << text2 << endl;

  // char to str
  char array[] = "here is text";
  string test = array;
  cout << test << endl;

  // prng
  cout << endl;
  lottery(49, 6);
  cout << endl;

  // typedef
  int test1 = 5;
  int test2 = 7;
  cout << (double)test1 / test2 << endl;                    // explicit c
  cout << double(test1 / test2) << endl;                    // explicit c
  cout << static_cast<double>(test1 / test2) << endl;       // explicit c++

  int varx = 184378923;                                     // implicit way can lose precision number can be off
  short vary = varx;
  cout << vary << endl;

  //strcmp(x,y) - 0 is eq 1 x is > -1 y is >
  /*for (int i = 0; i < argc; i++)
  {
    if (strcmp(argv[i], "-h") == 0 && i == 1)
      show_help();
  }*/

  personal_data person[2];
  person[0].age = 25;
  person[0].name = "ja";
  person[0].surname = "rel";
  person[0].number = "238189";

  mine.age = 30;
  yours.age = 31;

  cout << person[0].name << endl;
  cout << person[0].surname << endl;
  cout << person[0].age << endl;
  cout << person[0].number << endl;

  person[1].age = 35;
  person[1].name = "ma";
  person[1].surname = "rel";
  person[1].number = "1238189";

  cout << person[1].name << endl;
  cout << person[1].surname << endl;
  cout << person[1].age << endl;
  cout << person[1].number << endl;

  // retrieves the same
  cout << (*person).name << endl;
  cout << (*(person + 1)).name << endl;
  cout << (*&person[0]).name << endl;
  cout << person->name << endl;
  cout << (*person).name << endl;


  PersonalData personz;
  PersonalData personz2;

  personz.setAge(50);
  cout << personz.getAge() << endl;
  cout << personz2.getAge() << endl;

  // template
  cout << add(2, 5) << endl;
  // can specify type
  cout << add<double, double>(2,5.6) << endl;
  cout << static_cast<int>(6.5) << endl;

  // exceptions & handling
  // if we want to check or catch a specific result ex if b == 0 throw b  catch (...) will catch this instance
  int exc = 5;
  try
  {
    exc *= 10;
    if (exc == 50)
      throw exc;

  }
  catch (int e)
  {
    cout << "exc cannot be equal to " << exc << endl;
  }
  catch (...)
  {
    cout << "msg that is shown when other catch isn't invoked" << endl;
  }

  // namespaces
  cout << A::ns << endl;
  std::cout << "text: " << mySpace::endl.toString();


	system("pause");
}

int power(int b, int e)
{
  int tmp = b;
  while (e-- > 1)
  {
    b *= tmp;
  }

  return b;
}

double power(double b, int e)
{
  int tmp = b;
  while (e-- > 1)
  {
    b *= tmp;
  }

  return b;
}

void user_options()
{
  cout << "Enter option:" << endl;
  cout << "1. Circle"     << endl;
  cout << "2. Square"     << endl;
  cout << "3. Rectangle"  << endl;
  cout << "4. Triangle"   << endl;

}

void select_shape(int choice)
{
  double radius,x,y;
  int shape;
  switch (choice)
  {
    case 1:
      cout << endl;
      cout << "Enter the radius: " << endl;
      do { cin >> radius; } while (!is_valid("radius incorrect plz try again"));
      shape = 1;
      y = 0;
      calc_shape_area(radius, y, shape);
      break;
    case 2:
      cout << "Enter the side of the square" << endl;
      do { cin >> x; } while (!is_valid("side is incorrect plz try again"));
      shape = 2;
      y = 0;
      calc_shape_area(x, y, shape);
      break;
    case 3:
      cout << "Enter the width & length" << endl;
      do { cin >> x >> y; } while (!is_valid("width or length is incorrect plz try again"));
      shape = 3;
      calc_shape_area(x, y, shape);
      break;
    case 4:
      cout << "Enter the base & height " << endl;
      do { cin >> x >> y; } while (!is_valid("base or height is incorrect plz try again"));
      shape = 4;
      calc_shape_area(x, y, shape);
      break;
    default:
      cout << "You didn't choose a correct option" << endl;
  }

}

// would be interested to see how much memory is reserved when creating all the var vs separate functions for each shape
void calc_shape_area(double x, double y, int shape)
{
  double width = x, side= x, radius =x, base = x, height = y;

  switch (shape)
  {
  case 1:
    cout << "The area of the circle is: " << PI * radius * radius << endl;
    break;
  case 2:
    cout << "The area of the square is: " << side * side << endl;
    break;
  case 3:
    cout << "The area of the rectangle is: " << width * height << endl;
    break;
  case 4:
    cout << "The area of the triangle is: " << (base * height) / 2 << endl;
    break;
  default:
    cout << "shouldn't hit this";
  }

}

bool is_valid(string error_msg)
{
  if (cin.rdstate())                                         // state is wrong when eq 0
  {
    cin.clear();
    cin.ignore(numeric_limits<streamsize>::max(), '\n');    // ignore all but 1
    system("cls");
    user_options();
    cout << error_msg << endl;
    return false;
  }

  return true;

}

string getDay(dayOfWeek d)
{
  switch (d)
  {
  case M:
    return "Monday";
  case TU:
    return "Tuesday";
  case W:
    return "Wednesday";
  case TH:
    return "Thursday";
  case F:
    return "Friday";
  case SA:
    return "Saturday";
  case SU:
    return "Sunday";
  default:
    return "you typed something wrong";
  }
}

void swap(int &x, int &y) 
{
  int t;
  t = x;
  x = y;
  y = t;
}

void lottery(int total_balls, int balls_to_draw)
{
  if (total_balls < balls_to_draw)
    return;

  srand(time(NULL));

  int *balls = new int[balls_to_draw];
  for (int i = 0; i < balls_to_draw; i++)
  {
    balls[i] = rand() % 49 + 1;
    for (int j = 0; j < i +1; j++)
    {
      if (balls[i] == balls[j] && i != j)
      {
        i--;
        break;
      } else if (j==i)
        cout << balls[i] << endl;
    } 
  }

  delete[] balls;
}

void test_construc_destruct() 
{
  PersonalData person[5]; // or
  PersonalData *pointer = new PersonalData[5];

  delete[]pointer;        // free globally allocated memory
}

void change_data(PersonalData & obj, int value)
{
  obj.age = value;      // can't access age - private - make func friend
}