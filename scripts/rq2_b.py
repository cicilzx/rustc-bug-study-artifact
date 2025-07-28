import pandas as pd
from collections import defaultdict

# Load data
df = pd.read_csv('./data/all_issues.csv')

# Ensure necessary columns
required_cols = ['Status', 'Bug Cause', 'Symptoms']
if not all(col in df.columns for col in required_cols):
    raise ValueError("Missing required columns.")

# Filter valid entries
valid_df = df[df['Status'] == 'valid']

# Bug Cause → Cause Group mapping
cause_mapping = {
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

# Filter to valid bug causes
valid_df = valid_df[valid_df['Bug Cause'].isin(cause_mapping)]
valid_df['Cause Group'] = valid_df['Bug Cause'].map(cause_mapping)

# Extract Symptoms prefix
valid_df['Symptom Group'] = valid_df['Symptoms'].str.extract(r'^(1\.|2\.|3\.|4\.|5\.)')
valid_df = valid_df.dropna(subset=['Symptom Group'])

# Count (Symptom Group, Cause Group)
group_counts = defaultdict(lambda: defaultdict(int))

for _, row in valid_df.iterrows():
    symptom_group = row['Symptom Group']
    cause_group = row['Cause Group']
    group_counts[symptom_group][cause_group] += 1

# Cause group display order
cause_order = [
    "Type System Errors",
    "Ownership & Lifetime Errors",
    "MIR Optimization Errors",
    "General Errors"
]

# Output formatting
col1_width = 10
col2_width = 32
col3_width = 8
col4_width = 10
line_width = col1_width + col2_width + col3_width + col4_width + 4

print(f"{'Symptom Group'.ljust(col1_width)}{'Cause'.ljust(col2_width)}{'Count'.rjust(col3_width)}  {'Ratio'.rjust(col4_width)}")
print("-" * line_width)

for symptom_group in ["1.", "2.", "3.", "4.", "5."]:
    subtotal = sum(group_counts[symptom_group].values())
    if subtotal == 0:
        continue
    for cause in cause_order:
        count = group_counts[symptom_group].get(cause, 0)
        ratio = count / subtotal if subtotal > 0 else 0
        print(f"{symptom_group.ljust(col1_width)}{cause.ljust(col2_width)}{str(count).rjust(col3_width)}  {f'{ratio:.1%}'.rjust(col4_width)}")
    print(f"{'Subtotal'.ljust(col1_width)}{' '.ljust(col2_width)}{str(subtotal).rjust(col3_width)}  {f'{1.0:.1%}'.rjust(col4_width)}")
    print("-" * line_width)
