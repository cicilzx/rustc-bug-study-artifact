import csv 
import ast
from collections import Counter

# Ordered target label list (preserving order)
target_labels = [
    'A-HIR', 'A-THIR', 'A-MIR', 'A-mir-opt', 'A-mir-opt-inlining',
    'A-mir-opt-GVN', 'A-mir-opt-nrvo', 'A-stable-MIR', 'A-type-system',
    'A-inference', 'A-closures', 'A-coercions', 'A-const-generics',
    'A-DSTs', 'A-zst', 'A-trait-system', 'A-impl-trait', 'A-trait-objects',
    'A-auto-traits', 'A-implied-bounds', 'A-coinduction', 'A-coherence',
    'A-lifetimes', 'A-borrow-checker'
]

# Convert to set for faster lookup
target_label_set = set(target_labels)

# Insert a horizontal line after these labels
split_labels = {'A-THIR', 'A-stable-MIR', 'A-coherence'}

label_counter = Counter()

# Read primary issue labels
with open('./data/all_issues.csv', newline='', encoding='utf-8') as csvfile:
    reader = csv.DictReader(csvfile)
    for row in reader:
        labels_str = row.get('Issue Labels', '')
        if labels_str:
            try:
                if labels_str.startswith('"') and labels_str.endswith('"'):
                    labels_str = labels_str[1:-1]
                labels = ast.literal_eval(labels_str)
                for label in labels:
                    if label in target_label_set:
                        label_counter[label] += 1
            except Exception:
                print("Parse error, skipped:", labels_str)
                continue

# Print main output with separators
print(f"{'Label':<30}Count")
print("=" * 40)
for label in target_labels:
    print(f"{label:<30}{label_counter[label]}")
    if label in split_labels:
        print("-" * 40)

# Read backend issue labels
backend_labels = {'A-LLVM', 'A-gcc', 'A-cranelift'}
backend_counter = Counter()

with open('./data/backend_issues.csv', newline='', encoding='utf-8') as csvfile:
    reader = csv.DictReader(csvfile)
    for row in reader:
        label = row.get('tags', '')
        if label in backend_labels:
            backend_counter[label] += 1

# Print backend block
print("\nBackend (Excluded)")
print("=" * 40)
for label in ['A-LLVM', 'A-gcc', 'A-cranelift']:
    print(f"{label:<30}{backend_counter[label]}")
