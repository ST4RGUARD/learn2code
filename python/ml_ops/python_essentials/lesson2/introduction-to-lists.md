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

# Introduction to Python `lists`

In Python, lists are like a container where you can add items indefinitely. Lists is one of the most common data structures in Python. This notebook covers how to create lists and iterate over them

```python
# lists are containers of items identified by square brackets
[1, 2, 3, 4, 5]
```

```python
# lists can contain different types of items in them
[1, "two", False, 12.5]
```

```python
# you can use a built-in to count items in a list
len([3, 4, "red", "car"])
```

```python
# each item has a position called "index". Count starts with 0
items = ["carrots", "peas", "celery"]

# retrieving an item is done with the index
items[0]
```

```python
# you can also retrieve the last item 
items[-1]
```

```python
# but also use negative indexes to count from the end
items[-2]
```
