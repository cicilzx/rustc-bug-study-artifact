import pandas as pd
from collections import defaultdict

# Load data
df = pd.read_csv('./data/all_issues.csv')

# Only keep valid issues
df = df[df['Status'] == 'valid']

# Define mapping from Bug Cause to Cause Group
cause_to_group = {
    "1. Type-Trait & Bound": "Type System Errors",
    "1. Type-Opaque Types": "Type System Errors",
    "1. Type-New Solver": "Type System Errors",
    "1. Type-WF": "Type System Errors",

    "2. Ownership-Borrow&Move": "Ownership & Lifetime Errors",
    "2. Ownership-Lifetime": "Ownership & Lifetime Errors",

    "3. Opt-Wrong Implementations": "MIR Optimization Errors",
    "3. Opt-Missing Cases": "MIR Optimization Errors",

    "4. Basic syntax&structure": "General Errors",
    "5. Exception Handling & Report": "General Errors",
    "6. Compatibility Issues": "General Errors"
}

# Filter rows with known bug causes
df = df[df['Bug Cause'].isin(cause_to_group.keys())]

# Map Bug Cause to high-level Cause Group
df['Cause Group'] = df['Bug Cause'].map(cause_to_group)

# Only consider known sources
valid_sources = ['members', 'icemaker', 'developer', 'Rustlantis', 'fuzz-rustc']
df = df[df['Found by'].isin(valid_sources)]

# Count (Cause Group, Found by)
counts = defaultdict(lambda: defaultdict(int))
for _, row in df.iterrows():
    group = row['Cause Group']
    source = row['Found by']
    counts[group][source] += 1

# Formatting
col1_width = 32
col2_width = 15
col3_width = 8
line_width = col1_width + col2_width + col3_width

print(f"{'Cause Group'.ljust(col1_width)}{'Found by'.ljust(col2_width)}{'Count'.rjust(col3_width)}")
print("-" * line_width)

# Ensure consistent order
cause_groups = [
    "Type System Errors",
    "Ownership & Lifetime Errors",
    "MIR Optimization Errors",
    "General Errors"
]

for group in cause_groups:
    subtotal = 0
    label_printed = False
    for source in valid_sources:
        count = counts[group].get(source, 0)
        if count > 0:
            label = group if not label_printed else ""
            print(f"{label.ljust(col1_width)}{source.ljust(col2_width)}{str(count).rjust(col3_width)}")
            subtotal += count
            label_printed = True
    if subtotal > 0:
        print(f"{'Subtotal'.ljust(col1_width + col2_width)}{str(subtotal).rjust(col3_width)}")
        print("-" * line_width)
