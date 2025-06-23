import pandas as pd
from collections import defaultdict

# Load data
df = pd.read_csv('./data/all_issues.csv')

# Only keep valid issues
df = df[df['Status'] == 'valid']

# Define mapping from Symptoms prefix to full label
symptom_group_names = {
    '1.': '1. Crash',
    '2.': '2. Correctness Issues',
    '3.': '3. Miscompilation',
    '4.': '4. Diagnostic Issues',
    '5.': '5. Misoptimization',
}

# Extract symptom group prefix
df['Symptom Group'] = df['Symptoms'].str.extract(r'^(1\.|2\.|3\.|4\.|5\.)')
df = df.dropna(subset=['Symptom Group'])

# Only consider known sources
valid_sources = ['members', 'icemaker', 'developer', 'Rustlantis', 'fuzz-rustc']
df = df[df['Found by'].isin(valid_sources)]

# Count (Symptom Group, Found by)
counts = defaultdict(lambda: defaultdict(int))
for _, row in df.iterrows():
    group = row['Symptom Group']
    source = row['Found by']
    counts[group][source] += 1

# Formatting
col1_width = 28
col2_width = 15
col3_width = 8
line_width = col1_width + col2_width + col3_width

print(f"{'Symptom Group'.ljust(col1_width)}{'Found by'.ljust(col2_width)}{'Count'.rjust(col3_width)}")
print("-" * line_width)

for group_key in ['1.', '2.', '3.', '4.', '5.']:
    group_label = symptom_group_names[group_key]
    subtotal = 0
    for source in valid_sources:
        count = counts[group_key].get(source, 0)
        if count > 0:
            print(f"{group_label.ljust(col1_width)}{source.ljust(col2_width)}{str(count).rjust(col3_width)}")
            subtotal += count
            group_label = ''  # only print group label once
    if subtotal > 0:
        print(f"{'Subtotal'.ljust(col1_width + col2_width)}{str(subtotal).rjust(col3_width)}")
        print("-" * line_width)
