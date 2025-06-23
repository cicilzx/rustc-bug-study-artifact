import pandas as pd
from collections import defaultdict

# Read the CSV file
df = pd.read_csv('./all_issues.csv')

# Ensure the 'Bug Cause' column exists
if 'Bug Cause' not in df.columns:
    raise ValueError("The CSV file does not contain a 'Bug Cause' column")

# Count how many rows have Status == 'valid'
valid_total = (df['Status'] == 'valid').sum()

# Define the mapping from raw labels to display labels
cause_mapping = {
    "1. Type-Trait & Bound": "Trait & Bound",
    "1. Type-Opaque Types": "Opaque types",
    "1. Type-New Solver": "New solver",
    "1. Type-WF": "Well-formedness",
    
    "2. Ownership-Borrow&Move": "Borrow & Move",
    "2. Ownership-Lifetime": "Lifetime",

    "3. Opt-Wrong Implementations": "Wrong implementations",
    "3. Opt-Missing Cases": "Missing cases",

    "4. Basic syntax&structure": "Basic structure",
    "5. Exception Handling & Report": "Error handling & Reporting",
    "6. Compatibility Issues": "Compatibility"
}

# Count Bug Cause occurrences
cause_counts = df['Bug Cause'].value_counts()

# Group causes into 4 groups
grouped = defaultdict(list)
for raw_label, display_label in cause_mapping.items():
    count = cause_counts.get(raw_label, 0)
    ratio = count / valid_total
    if raw_label.startswith("1."):
        group_key = "Group 1"
    elif raw_label.startswith("2."):
        group_key = "Group 2"
    elif raw_label.startswith("3."):
        group_key = "Group 3"
    else:
        group_key = "Group 4-6"
    grouped[group_key].append((display_label, count, ratio))

# Determine formatting widths
col1_width = max(len(label) for group in grouped.values() for label, _, _ in group) + 2
col2_width = 10
col3_width = 8
line_width = col1_width + col2_width + col3_width + 4

# Print table header
print(f"{'Bug Cause'.ljust(col1_width)}{'Count'.rjust(col2_width)}  {'Ratio'.rjust(col3_width)}")
print("-" * line_width)

# Print grouped results with subtotals
grand_total = 0
group_order = ["Group 1", "Group 2", "Group 3", "Group 4-6"]

for group_name in group_order:
    group = grouped.get(group_name, [])
    subtotal = 0
    subtotal_ratio = 0.0
    for label, count, ratio in group:
        print(f"{label.ljust(col1_width)}{str(count).rjust(col2_width)}  {f'{ratio:.2%}'.rjust(col3_width)}")
        subtotal += count
        subtotal_ratio += ratio
    print(f"{'Subtotal'.ljust(col1_width)}{str(subtotal).rjust(col2_width)}  {f'{subtotal_ratio:.2%}'.rjust(col3_width)}")
    print("-" * line_width)
    grand_total += subtotal

# Print final total
print(f"{'Total'.ljust(col1_width)}{str(grand_total).rjust(col2_width)}  {f'{grand_total / valid_total:.2%}'.rjust(col3_width)}")