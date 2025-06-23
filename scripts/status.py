import pandas as pd

# Read the CSV file
df = pd.read_csv('./data/all_issues.csv')

# Ensure the 'Status' column exists
if 'Status' not in df.columns:
    raise ValueError("The CSV file does not contain a 'Status' column")

# Define the mapping from raw CSV labels to display labels
status_mapping = {
    'duplicate': 'Duplicate',
    'Not a bug': 'Not a bug',
    'Not reproducible anymore': 'Not reproducible',
    'question': 'Discussion',
    'exclude': 'Exclude',
    'valid': 'Valid'
}

# Count the occurrences of each raw status
status_counts = df['Status'].value_counts()

# Prepare the display results with counts
results = []
for raw_label, display_label in status_mapping.items():
    count = status_counts.get(raw_label, 0)
    results.append((display_label, count))

# Calculate total count
total = sum(count for _, count in results)

# Determine column widths for alignment
col1_width = max(len(label) for label, _ in results) + 2
col2_width = 10
line_width = col1_width + col2_width + 2

# Print the formatted table
print(f"{'Status'.ljust(col1_width)}{'Count'.rjust(col2_width)}")
print("-" * line_width)
for label, count in results:
    print(f"{label.ljust(col1_width)}{str(count).rjust(col2_width)}")
print("-" * line_width)
print(f"{'Total'.ljust(col1_width)}{str(total).rjust(col2_width)}")
