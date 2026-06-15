---
jupyter:
  jupytext:
    text_representation:
      extension: .md
      format_name: markdown
      format_version: '1.3'
      jupytext_version: 1.17.2
  kernelspec:
    display_name: Python 3
    language: python
    name: python3
---

<!-- #region id="wQIK5-fWGir3" -->
### Understanding Python Classes


<!-- #endregion -->

<!-- #region id="rMA9KJbaG1T2" -->
#### Differences Between Classes and Functions
The key differences between Classes and Functions are:

* Functions are much easier to reason about
* Functions (typically) have state inside the function only, where classes have state persists outside of the function
* Classes can offer a more advanced level of abstraction at the cost of complexity
<!-- #endregion -->

<!-- #region id="9DsnmUoyHGzx" -->
#### Creating an empty Class
Using classes and interacting with them can be done iteratively in Jupyter Notebook.
The simplest type of class is just a name as shown below:
```
class Competitor: pass
```
But, that class can be instantiated into multiple objects
<!-- #endregion -->

```python
class Competitor: pass
```

<!-- #region id="7bNTtyGvHh7E" -->
#### Setting Attributes on an Object
<!-- #endregion -->

```python
conor = Competitor()
conor.name = "Conor McGregor"
conor.age = 29
conor.weight = 155

conor.__dict__

```

```python
nate = Competitor()
nate.name = "Nate Diaz"
nate.age = 30
nate.weight = 170
nate.__dict__
```

<!-- #region id="jzJzVnWDHo2A" -->
#### Interacting with Objects
<!-- #endregion -->

```python
def print_competitor_age(object):
    """Print out age statistics about a competitor"""

    print(f"{object.name} is {object.age} years old")
```

```python
print_competitor_age(nate)
```

```python
print_competitor_age(conor)
```

<!-- #region id="yplytoRKI8pA" -->
#### Understanding Inheritance
Classes can also inhert from other classes including methods.
Often inheritance can be complex and a rule of thumb is to use discretion.

In the example below, a UFC class was created that has a method (similar to a function), that can determine what weight class an athlete belongs to.  Then the Competitor class uses "inheritance", to inhert the code in the class.
<!-- #endregion -->

<!-- #region id="jJ2j1P-XJDVi" -->
##### Using Inheritance
<!-- #endregion -->

```python
class UFC:
    def weight_class(self, weight):
        """Weight Class Finder"""

        classes = {155: "Lightweight",
                    170: "Welterweight"}
        return classes[weight]


```

```python
class Competitor(UFC): pass
```

```python
conor = Competitor()
conor.name = "Conor McGregor"
conor.age = 29
conor.weight = 155

```

```python
nate = Competitor()
nate.name = "Nate Diaz"
nate.age = 30
nate.weight = 170

```

```python
conor.weight_class
```

<!-- #region id="Uqkx_uN3JQxV" -->
##### Using inherited methods from Parent Class
<!-- #endregion -->

```python
print(conor.weight_class(conor.weight))
```

```python
print(nate.weight_class(nate.weight))
```

<!-- #region id="PEKWYhudJv_x" -->
#### Using Multiple Inheritance

Multiple Inheritance is inheriting more than one class
<!-- #endregion -->

```python
class MMA:
  def org(self, org_name):
      orgs = {"UFC": "Ultimate Fighting Championship",
          "Bellator":  "MMA promotion in Santa Monica, California."}
      return orgs[org_name]
```

```python
class CompetitorAll(UFC, MMA):pass
```

```python
gsp = CompetitorAll()
gsp.name = "GSP"
gsp.age = 27
gsp.weight = 170
print(f'{gsp.name} is the G.O.A.T in the {gsp.weight_class(gsp.weight)} division of the {gsp.org("UFC")}')
```

<!-- #region id="srx3voPVKPop" -->
#### Interacting with Special Class Methods and Other Class Techniques

Class special methods have the signature ```__method__```:

Examples include
```
__len__
__call__
__equal__

```
<!-- #endregion -->

```python
l = [1,2]
len(l)
#class Foo:pass
#f = Foo()
#len(f)
```

```python
class JonJones:
  """Jon Jones class with customized length"""

  def __len__(self):
    return 84

jon_jones = JonJones()
len(jon_jones)
```

```python
class foo():pass
f = foo()
f.red = "red"
len(f)
```

<!-- #region id="tbTyE8mrKvcj" -->
@property decorator is a shortcut for creating a read only property
<!-- #endregion -->

```python
class JonJones:
  """Jon Jones class with read only property"""

  @property
  def reach(self):
    return 84

jon_jones = JonJones()
jon_jones.reach
#jon_jones.reach = 85 #cannot set
jon_jones.length = 85
jon_jones.length
```

<!-- #region id="grcojpMOKyao" -->
@staticmethod bolts a function onto a class
<!-- #endregion -->

```python
class JonJones:
  """Jon Jones Class with 'bolt-on' reach method
  self isn't needed
  """

  @staticmethod
  def reach():
    return 84

jon_jones =JonJones()
jon_jones.reach()
```

<!-- #region id="dhabI3o_dWd7" -->
#### Immutability concepts with Objects
<!-- #endregion -->

```python
class Foo:

  @property
  def unbreakable(self):
    return "David"


```

```python
foo = Foo()
foo.unbreakable
```

```python
foo.not_unbreakable = "Elijah2"
```

<!-- #region id="VuSZ9Y49egiO" -->
@property acts like an read only attribute, but it isn't
<!-- #endregion -->

```python
foo.__dict__
```

<!-- #region id="DmLJWuUtfE2-" -->
You can change an attribute on the object, but not the read only property
<!-- #endregion -->

```python
foo.not_unbreakable = "Mr. Glass"
```

```python
foo.unbreakable = "Bruce Willis"
```
