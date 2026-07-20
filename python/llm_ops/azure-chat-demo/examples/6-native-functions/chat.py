from dotenv import load_dotenv
import os
import asyncio

from azure.identity.aio import DefaultAzureCredential
from azure.ai.inference.aio import ChatCompletionsClient

import semantic_kernel as sk
from semantic_kernel.connectors.ai.azure_ai_inference import (
    AzureAIInferenceChatCompletion,
)
from semantic_kernel.functions import kernel_function


class TravelWeather:
    @kernel_function(
        name="travel_weather",
        description="Returns the average temperature for a city and month.",
    )
    def weather(
        self,
        city: str,
        month: str,
    ) -> str:
        return (
            f"The average temperature in {city} during {month} "
            f"is 75 degrees. This is an amazing choice!"
        )


async def main():
    load_dotenv()

    deployment = os.environ["AZURE_MODEL_NAME"]
    endpoint = os.environ["AZURE_AI_ENDPOINT"].rstrip("/")

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
            instruction_role="developer",
        )
    )

    kernel.add_plugin(
        TravelWeather(),
        plugin_name="TravelWeather",
    )

    prompt = """
    You are a travel weather chatbot named Frederick.

    Help the user find the average temperature for their destination.
    Use the travel_weather function when you need temperature information.

    User request:
    {{$input}}
    """

    result = await kernel.invoke_prompt(
        prompt,
        input=(
            "I'm travelling to Lima, and it seems that it would happen "
            "in August. What would be the average temperature?"
        ),
    )

    print(result)

    await client.close()
    await credential.close()


if __name__ == "__main__":
    asyncio.run(main())
