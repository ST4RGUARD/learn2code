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

## Introduction to dictionaries

Python dictionaries are a great way to store data and it is as common as using lists. The dictionary is used to map keys with values, creating a mapping of items.

```python
# curly brackets are required to create one
{}
```

```python
# you always map a key to a value
{"key": "value"}
```

```python
# the values can be other types 
{"key": True}
```

```python
# but the key has to be unique, there can't be duplicates
{"name": "Alfredo", "name": "Alfredo"}
```

```python
# value can be other dictionaries or lists
{"items": ["lumber", "concrete", "nails"]}
```

```python
# you can't have a list or a dict as a key however!
{[1,2]: False}
```
