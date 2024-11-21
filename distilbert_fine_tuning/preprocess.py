import pandas as pd

# Load the dataset (adjust the file path and type as needed)
df = pd.read_csv('/home/five/fine-tuning-data/stackexchange_bitcoin_queryresults_top10K_filtered.csv')  # or pd.read_json('path/to/your_dataset.json')

# Inspect the data structure
print('Data structure:')
print(df.head())

# Check for missing values
print('Missing values')
print(df.isnull().sum())

# Whitelist specific columns and drop the rest
df = df[['Id', 'Title', 'Body', 'Tags']]

