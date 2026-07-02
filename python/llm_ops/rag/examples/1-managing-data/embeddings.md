---
jupyter:
  jupytext:
    text_representation:
      extension: .md
      format_name: markdown
      format_version: '1.3'
      jupytext_version: 1.19.4
  kernelspec:
    display_name: Python 3 (ipykernel)
    language: python
    name: python3
---

```python
import pandas as pd
df = pd.read_csv('../../top_rated_wines.csv')
df = df[df['variety'].notna()] # remove any NaN values as it blows up serialization
data = df.to_dict('records')
df
```

```python
from qdrant_client import models, QdrantClient
from sentence_transformers import SentenceTransformer
```

```python
encoder = SentenceTransformer('all-MiniLM-L6-v2') # Model to create embeddings
```

```python
# create the vector database client
qdrant = QdrantClient(":memory:") # Create in-memory Qdrant instance
```

```python
# Create collection to store books
qdrant.recreate_collection(
    collection_name="top_wines",
    vectors_config=models.VectorParams(
        size=encoder.get_sentence_embedding_dimension(), # Vector size is defined by used model
        distance=models.Distance.COSINE
    )
)
```

```python
# vectorize!
qdrant.upload_points(
    collection_name="top_wines",
    points=[
        models.PointStruct(
            id=idx,
            vector=encoder.encode(doc["notes"]).tolist(),
            payload=doc
        ) for idx, doc in enumerate(data) # data is the variable holding all the wines
    ]
)
```

```python
# Search time for awesome wines!

# Perform the query
response = qdrant.query_points(
    collection_name="top_wines",
    query=encoder.encode("99 points Cabernet Sauvignon from Napa Valley").tolist(),
    limit=3
)

# Access the points list from the response object
for hit in response.points:
    # hit is a ScoredPoint object
    print(f"Payload: {hit.payload}")
    print(f"Score: {hit.score}")
    print("-" * 20)
 ```

```python

```
