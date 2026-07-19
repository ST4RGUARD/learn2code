from dotenv import load_dotenv
import os
import asyncio

from azure.identity.aio import DefaultAzureCredential
from azure.ai.inference.aio import ChatCompletionsClient

import semantic_kernel as sk
from semantic_kernel.connectors.ai.azure_ai_inference import (
    AzureAIInferenceChatCompletion,
)


async def main():
    load_dotenv()

    endpoint = os.environ["AZURE_AI_ENDPOINT"].rstrip("/")
    deployment = os.environ["AZURE_MODEL_NAME"]

    credential = DefaultAzureCredential()

    client = ChatCompletionsClient(
        endpoint=f"{endpoint}/openai/deployments/{deployment}",
        credential=credential,
        credential_scopes=[
            "https://cognitiveservices.azure.com/.default",
        ],
    )

    kernel = sk.Kernel()

    kernel.add_service(
        AzureAIInferenceChatCompletion(
            service_id="chat",
            ai_model_id=deployment,
            client=client,
            instruction_role="developer",  # Recommended for reasoning models
        )
    )

    result = await kernel.invoke_prompt(
        "Explain espresso extraction in three paragraphs."
    )

    print(result)

    await client.close()
    await credential.close()


if __name__ == "__main__":
    asyncio.run(main())
