import pandas as pd
from collections import defaultdict

# Read CSV
df = pd.read_csv('./data/all_issues.csv')

# Required columns
required_cols = ['Bug Cause', 'Status', 'Stage']
if not all(col in df.columns for col in required_cols):
    raise ValueError("Missing required columns.")

# Filter to valid issues
valid_df = df[df['Status'] == 'valid']

# Map bug causes to groups
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

valid_df = valid_df[valid_df['Bug Cause'].isin(cause_to_group)]
valid_df['Cause Group'] = valid_df['Bug Cause'].map(cause_to_group)

# Custom stage order
target_stages = [
    "2. HIR-type & wf check",
    "2. HIR-trait solving",
    "2. HIR-Type Infer",
    "2. HIR-report",
    "3. MIR-MIR Transform",
    "3. MIR-Borrow Check",
    "3. MIR-Dataflow Analysis",
    "3. MIR-report",
]

cause_groups = [
    "Type System Errors",
    "Ownership & Lifetime Errors",
    "MIR Optimization Errors",
    "General Errors"
]

# Output formatting
col1_width = max(len(stage) for stage in target_stages + ['Subtotal', 'Total']) + 2
col2_width = max(len(cause) for cause in cause_groups + ['Subtotal', 'Total']) + 2
col3_width = 10
line_width = col1_width + col2_width + col3_width + 4

print(f"{'Stage'.ljust(col1_width)}{'Cause'.rjust(col2_width)}  {'Ratio'.rjust(col3_width)}")
print("-" * line_width)

# Group into phases
phase_to_stages = {
    "2. HIR": [s for s in target_stages if s.startswith("2. HIR-")],
    "3. MIR": [s for s in target_stages if s.startswith("3. MIR-")],
}

grand_total = 0

for phase_name, stages in phase_to_stages.items():
    phase_df = valid_df[valid_df['Stage'].isin(stages)]
    phase_total = len(phase_df)

    for stage in stages:
        stage_df = phase_df[phase_df['Stage'] == stage]
        stage_subtotal = 0
        for cause in cause_groups:
            count = len(stage_df[stage_df['Cause Group'] == cause])
            ratio = count / phase_total if phase_total > 0 else 0
            stage_subtotal += count
            print(f"{stage.ljust(col1_width)}{cause.rjust(col2_width)}  {f'{ratio:.2%}'.rjust(col3_width)}")
        print(f"{'Subtotal'.ljust(col1_width)}{str(stage_subtotal).rjust(col2_width)}  {f'{stage_subtotal / phase_total:.2%}'.rjust(col3_width)}")
        print("-" * line_width)
        grand_total += stage_subtotal
