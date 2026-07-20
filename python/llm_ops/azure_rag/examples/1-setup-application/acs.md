---
jupyter:
  jupytext:
    text_representation:
      extension: .md
      format_name: markdown
      format_version: '1.3'
      jupytext_version: 1.19.4
  kernelspec:
    display_name: rag
    language: python
    name: python3
---

```python
import os

from dotenv import load_dotenv

from azure.identity import DefaultAzureCredential, get_bearer_token_provider

from langchain_openai import AzureOpenAIEmbeddings
from langchain_community.vectorstores import AzureSearch

load_dotenv()


credential = DefaultAzureCredential()

token_provider = get_bearer_token_provider(
    credential,
    "https://ai.azure.com/.default",
)
```

```python
embeddings = AzureOpenAIEmbeddings(
    azure_endpoint=os.environ["AZURE_AI_ENDPOINT"],
    azure_ad_token_provider=token_provider,
    api_version="2024-10-21",
    azure_deployment=os.environ["AZURE_AI_EMBEDDING_DEPLOYMENT"],
    chunk_size=1,
)
```

```python
from langchain_community.document_loaders import CSVLoader

loader = CSVLoader("wine-ratings.csv")
documents = loader.load()
```

```python
acs = AzureSearch(
    azure_search_endpoint=os.getenv("SEARCH_SERVICE_NAME"),
    azure_search_key=os.getenv("SEARCH_API_KEY"),
    index_name=os.getenv("SEARCH_INDEX_NAME"),
    embedding_function=embeddings.embed_query,
)
```

```python
from langchain_text_splitters import CharacterTextSplitter

text_splitter = CharacterTextSplitter(chunk_size=1000, chunk_overlap=0)

# for testing, we will only add 1000 rows to the index
documents = documents[:1000]
docs = text_splitter.split_documents(documents)

acs.add_documents(documents=docs)
```

```python
docs = acs.similarity_search_with_relevance_scores(
    query="What is the best Cabernet Sauvignon wine in Napa Valley above 94 points",
    k=5,
)
print(docs[0][0].page_content)
print(dir(docs[0][0]))
```


```python
from openai import OpenAI

client = OpenAI(
    base_url=os.environ["AZURE_AI_ENDPOINT"] + "/openai/v1",
    api_key=token_provider,
)


context = "\n\n".join(
    [doc[0].page_content for doc in docs]
)


messages = [
    {
        "role": "system",
        "content": (
            "You are a wine assistant. "
            "Answer using only the provided context."
        ),
    },
    {
        "role": "user",
        "content": (
            f"""
Context:
{context}

Question:
What is the best wine in Oregon above 92 points?
"""
        ),
    },
]


response = client.chat.completions.create(
    model=os.environ["AZURE_CHAT_DEPLOYMENT"],
    messages=messages,
)


print(response.choices[0].message.content)

```


