---
jupyter:
  jupytext:
    text_representation:
      extension: .md
      format_name: markdown
      format_version: '1.3'
      jupytext_version: 1.19.4
  kernelspec:
    display_name: Python 3
    language: python
    name: python3
---

## Example Notebook using local ai to pull the FLUX.1-schnell model down from Hugging Face

```python
import os
import re
import torch
from openai import OpenAI
from diffusers import AutoPipelineForText2Image

# 1. Setup your Local LLM Client (for text generation)
# Point this to your qwen / llamafile text server (assuming port 8080)
text_client = OpenAI(
    base_url="http://localhost:8080/v1",
    api_key="not-needed")
```

```python
# Create shopping list and recipe function
def create_shopping_list(recipe):
    prompt = f"Create a wizard with the following color hat: {(recipe)}"

    return prompt
```

```python
# Function Test
recipe = create_shopping_list("red")

print(recipe)
```

```python
# Reaching out to your LOCAL text model for the answer
response = text_client.chat.completions.create(
    model="qwen", # Use your local qwen model identifier here
    messages=[
        {
            "role": "user", # Swapped to user role so the model responds to the prompt
            "content": recipe
        },
    ],
    temperature=0.7,
    top_p=1,
)

recipe_text = response.choices[0].message.content
print(recipe_text)
```

```python
# Putting the ingredients into a shopping list

# Split the response text into individual lines
lines = recipe_text.strip().split('\n')

shopping_list = []

for line in lines:
    clean_line = line.strip()
    
    # Match lines that start with a bullet point (* or -) followed by bold text
    # This regex pulls out just the text inside the first set of double asterisks
    match = re.search(r'^[\*\-]\s+\*\*([^*]+)\*\*', clean_line)
    
    if match:
        ingredient = match.group(1).strip()
        shopping_list.append(ingredient)

# Safety check and output
if shopping_list:
    print("Parsed Shopping List:", shopping_list)
    print("\nItem to be imaged:", shopping_list[0])
else:
    print("Could not parse items. Falling back to a clean default prompt.")
    shopping_list.append("Chipotle chicken and rice bowl ingredients")
    print("Item to be imaged:", shopping_list[0])
#
#
#pattern = re.compile(r'- (.+)')
#matches = pattern.findall(recipe_text)

#shopping_list = []
#for match in matches:
#    shopping_list.append(match)

#print("Item to be imaged:", shopping_list[0])
```

```python
# ========================================================
# CELL 6: NATIVE M3 IMAGE GENERATION (JUGGERNAUT-XL VIA MPS)
# ========================================================
print("🚀 Loading Juggernaut-XL natively into M3 Unified Memory...")

# 1. Initialize the clean, pipeline framework
pipe = AutoPipelineForText2Image.from_pretrained(
    "RunDiffusion/Juggernaut-XL-v9", 
    torch_dtype=torch.float16, 
    variant="fp16"
)
pipe = pipe.to("mps")

# 2. Reconstruct the target recipe text prompt strings
target_item = "wiazard with a red hat"
image_prompt = f"A high quality photorealistic studio shot of {target_item}, professional magazine photography, 8k resolution, crisp focus, intricate textures"

print(f"🎨 Generating image for: '{target_item}' using Metal GPU acceleration...")

# 3. Process the model generation weights
image = pipe(
    prompt=image_prompt, 
    num_inference_steps=30, 
    guidance_scale=7.0 
).images[0]

# 4. Save a physical copy to your disk so you can view it natively anytime
output_path = "recipe_output.png"
image.save(output_path)
print(f"💾 High-res image safely saved to disk at: {output_path}")

# 5. Native Jupyter Notebook display call
# (Will evaluate silently in the terminal, but embeds perfectly in your .ipynb file)
display(image)
```

