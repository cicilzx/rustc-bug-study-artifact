import pandas as pd
from collections import defaultdict

# Read data
df = pd.read_csv('./all_issues.csv')

# Check necessary columns
required_cols = ['Bug Cause', 'Status', 'Stage']
if not all(col in df.columns for col in required_cols):
    raise ValueError("CSV must contain 'Bug Cause', 'Status', and 'Stage' columns")

# Filter to valid issues only
valid_df = df[df['Status'] == 'valid']
valid_total = len(valid_df)
if valid_total == 0:
    raise ValueError("No valid issues found.")

# Define Bug Cause → Group
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

# Filter out unmapped Bug Causes
valid_df = valid_df[valid_df['Bug Cause'].isin(cause_to_group.keys())]
valid_df['Cause Group'] = valid_df['Bug Cause'].map(cause_to_group)

# Map Stage to Stage Group
def map_stage(stage):
    if stage == "1. AST":
        return "1. AST"
    elif stage.startswith("2. HIR-"):
        return "2. HIR"
    elif stage.startswith("3. MIR-"):
        return "3. MIR"
    elif stage == "4. Code Gen":
        return "4. Code Gen"
    elif stage.startswith("5. Utils"):
        return "5. Utility"
    elif stage == "6. LLVM":
        return "6. LLVM"
    else:
        return None

valid_df['Stage Group'] = valid_df['Stage'].apply(map_stage)
valid_df = valid_df.dropna(subset=['Stage Group'])

# Count (Stage Group, Cause Group) occurrences
stage_cause_counts = defaultdict(lambda: defaultdict(int))
for _, row in valid_df.iterrows():
    stage = row['Stage Group']
    cause = row['Cause Group']
    stage_cause_counts[stage][cause] += 1

# Define display order
stage_groups = ["1. AST", "2. HIR", "3. MIR", "4. Code Gen", "5. Utility", "6. LLVM"]
cause_groups = ["Type System Errors", "Ownership & Lifetime Errors", "MIR Optimization Errors", "General Errors"]

# Output formatting
# Dynamically calculate column widths based on longest strings
col1_width = max(len(stage) for stage in stage_groups) + 2
col2_width = max(len(cause) for cause in cause_groups + ["Subtotal", "Total"]) + 2
col3_width = 10  # Ratio column can remain fixed
line_width = col1_width + col2_width + col3_width + 4

print(f"{'Stage'.ljust(col1_width)}{'Cause'.rjust(col2_width)}  {'Ratio'.rjust(col3_width)}")
print("-" * line_width)

grand_total = 0
for stage in stage_groups:
    subtotal = 0
    for cause in cause_groups:
        count = stage_cause_counts[stage].get(cause, 0)
        ratio = count / valid_total
        subtotal += count
        print(f"{stage.ljust(col1_width)}{cause.rjust(col2_width)}  {f'{ratio:.2%}'.rjust(col3_width)}")
    print(f"{'Subtotal'.ljust(col1_width)}{str(subtotal).rjust(col2_width)}  {f'{subtotal / valid_total:.2%}'.rjust(col3_width)}")
    print("-" * line_width)
    grand_total += subtotal

# Print grand total
print(f"{'Total'.ljust(col1_width)}{str(grand_total).rjust(col2_width)}  {f'{grand_total / valid_total:.2%}'.rjust(col3_width)}")
