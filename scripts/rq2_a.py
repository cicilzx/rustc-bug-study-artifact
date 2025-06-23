import pandas as pd

# Load CSV
df = pd.read_csv('./all_issues.csv')

# Ensure required columns exist
required_cols = ['Symptoms', 'Status', 'Test validity']
if not all(col in df.columns for col in required_cols):
    raise ValueError("CSV must contain 'Symptoms', 'Status', and 'Test validity' columns")

# Filter only valid issues
valid_df = df[df['Status'] == 'valid']
valid_total = len(valid_df)
if valid_total == 0:
    raise ValueError("No valid 'Status' == 'valid' entries")

# Symptoms of interest
target_symptoms = [
    "1. Crash-Front-end Panic",
    "1. Crash-Back-end Crash",
    "2. Correctness Issues-Completeness Issues",
    "2. Correctness Issues-Soundness Issues",
    "3. Miscompilation-Inconsistent Output Issues",
    "3. Miscompilation-Safe Rust Causes UB",
    "4. Diagnostic Issues-Incorrect Warning/Error",
    "4. Diagnostic Issues-Improper Fix Suggestion",
    "5. Misoptimization"
]

# Group symptoms by prefix
symptom_groups = {
    "1.": ["1. Crash-Front-end Panic", "1. Crash-Back-end Crash"],
    "2.": ["2. Correctness Issues-Completeness Issues", "2. Correctness Issues-Soundness Issues"],
    "3.": ["3. Miscompilation-Inconsistent Output Issues", "3. Miscompilation-Safe Rust Causes UB"],
    "4.": ["4. Diagnostic Issues-Incorrect Warning/Error", "4. Diagnostic Issues-Improper Fix Suggestion"],
    "5.": ["5. Misoptimization"]
}

# Symptoms to split by Test validity
split_by_test_validity = {"1. Crash-Front-end Panic"}

# Output formatting setup
col1_width = max(len(s + "(invalid)") for s in target_symptoms + ["Subtotal", "Total"]) + 2
col2_width = 8
col3_width = 10
line_width = col1_width + col2_width + col3_width + 4

print(f"{'Symptom'.ljust(col1_width)}{'Count'.rjust(col2_width)}  {'Ratio'.rjust(col3_width)}")
print("-" * line_width)

grand_total = 0

# Grouped output
for group_prefix, symptoms in symptom_groups.items():
    subtotal = 0
    for s in symptoms:
        if s in split_by_test_validity:
            for vtype in ['valid', 'invalid']:
                label = f"{s}({vtype})"
                count = len(valid_df[(valid_df['Symptoms'] == s) & (valid_df['Test validity'] == vtype)])
                ratio = count / valid_total if valid_total > 0 else 0
                subtotal += count
                print(f"{label.ljust(col1_width)}{str(count).rjust(col2_width)}  {f'{ratio:.2%}'.rjust(col3_width)}")
        else:
            count = len(valid_df[valid_df['Symptoms'] == s])
            ratio = count / valid_total if valid_total > 0 else 0
            subtotal += count
            print(f"{s.ljust(col1_width)}{str(count).rjust(col2_width)}  {f'{ratio:.2%}'.rjust(col3_width)}")
    print(f"{'Subtotal'.ljust(col1_width)}{str(subtotal).rjust(col2_width)}  {f'{subtotal / valid_total:.2%}'.rjust(col3_width)}")
    print("-" * line_width)
    grand_total += subtotal

# Print grand total
print(f"{'Total'.ljust(col1_width)}{str(grand_total).rjust(col2_width)}  {f'{grand_total / valid_total:.2%}'.rjust(col3_width)}")
