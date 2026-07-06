import os
import pandas as pd
from fastapi import FastAPI
from fastapi.responses import RedirectResponse
from pydantic import BaseModel
from openai import OpenAI
from langchain_openai import OpenAIEmbeddings
from langchain_qdrant import QdrantVectorStore
from qdrant_client import QdrantClient
from langchain_core.documents import Document
from qdrant_client.http.models import Distance, VectorParams

app = FastAPI()

# 1. Setup local OpenAI-compatible client and embeddings
base_url = os.getenv("OPENAI_BASE_URL", "http://host.docker.internal:8080/v1")
client = OpenAI(base_url=base_url, api_key="not-needed")

embeddings = OpenAIEmbeddings(
    base_url=base_url,
    openai_api_key="not-needed",
    model="mxbai-embed-large-v1"
)

# 2. Setup Qdrant Client (pointing to a local file database)
client_qdrant = QdrantClient(path="./local_qdrant_db")

# 3. Lazy Initialization function to safely build or connect to the store
def get_vector_store():
    collection_name = "wine_collection"
    
    if not client_qdrant.collection_exists(collection_name):
        print("Initializing collection and loading data from wine-ratings.csv...")
        
        # 1. Explicitly create the collection with the 1024 dimensions required by mxbai
        client_qdrant.create_collection(
            collection_name=collection_name,
            vectors_config=VectorParams(
                size=1024,  # EXACT dimensions for mxbai-embed-large
                distance=Distance.COSINE
            )
        )
        
        # 2. Parse the documents from your CSV
        df = pd.read_csv("wine-ratings.csv")
        documents = [
            Document(page_content=f"Wine: {row['name']}, Rating: {row['rating']}, Region: {row['region']}, Notes: {row['notes']}")
            for _, row in df.iterrows()
        ]
        
        # 3. Instantiate the LangChain wrapper safely now that the index exists
        vector_store = QdrantVectorStore(
            client=client_qdrant,
            collection_name=collection_name,
            embedding=embeddings
        )
        
        # 4. Ingest your wine-ratings matrix
        vector_store.add_documents(documents)
        return vector_store
    else:
        print("Collection already exists. Connecting directly...")
        return QdrantVectorStore(
            client=client_qdrant,
            collection_name=collection_name,
            embedding=embeddings
        )

# Initialize vector_store safely at app startup
vector_store = get_vector_store()

class Body(BaseModel):
    query: str


@app.get('/')
def root():
    return RedirectResponse(url='/docs', status_code=301)


@app.post('/ask')
def ask(body: Body):
    search_result = search(body.query)
    chat_bot_response = assistant(body.query, search_result)
    return {'response': chat_bot_response}


def search(query):
    """
    Send the query to the local Qdrant Vector Store and return top hits
    """
    docs = vector_store.similarity_search(query, k=5)
    result = "\n\n".join([d.page_content for d in docs])
    print("--- Search Context Retrieved ---")
    print(result)
    return result


def assistant(query, context):
    messages = [
        {"role": "system", "content": "Assistant is a chatbot that helps you find the best wine for your taste using the context provided."},
        {"role": "user", "content": f"Context From Vector Database:\n{context}\n\nUser Question: {query}"}
    ]

    response = client.chat.completions.create(
        model="local-model",
        messages=messages,
    )
    return response.choices[0].message.content
