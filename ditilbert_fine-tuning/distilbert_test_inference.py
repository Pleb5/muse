from transformers import pipeline

# Load the pre-trained model for text classification
classifier = pipeline("text-classification", model="distilbert-base-uncased")

# Run inference on a sample input
text = "I need help with my account billing."
result = classifier(text)
print(result)

