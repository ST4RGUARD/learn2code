---
jupyter:
  jupytext:
    text_representation:
      extension: .md
      format_name: markdown
      format_version: '1.3'
      jupytext_version: 1.17.2
  kernelspec:
    display_name: Python 3.8.9 64-bit
    language: python
    name: python3
---

# Creating and iterating lists
You can create lists in a few different ways. Looping over a list is also one of the most common operations in Python. This notebook will show you several ways to create a list and then loop over its data.


## Creating lists
These are some of the different ways you can use to create a list

```python
# create one by defining an empty list with bracket
items = []
items
```

```python
# but you can also use the built-in `list()` in Python is more common to use brackets
items = list()
items
```

```python
# data can be pre-seeded
colors = ["red", "blue", "brown"]
colors
```

## Iterating over lists
This is one of the most common loop operations in Python

```python
for color in colors:
    print(color)
```

```python
# it is equivalent
for color in ["red", "yellow"]:
    print(color)
```

## List comprehensions
This is a more advanced way to loop over a list and set a condition. Use it sparingly and avoid it if the statement becomes too long

```python
numbers = [2, 3, 4, 12, 5, 3, 4]

low_numbers = [n for n in numbers if n < 6]

low_numbers
```
