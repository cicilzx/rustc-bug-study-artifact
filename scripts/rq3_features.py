import csv
from collections import Counter

def print_unstable_features(input_file):
    valid_count = 0  # Total number of valid rows
    valid_with_unstable_count = 0  # Valid rows with non-empty "unstable features"
    all_features = []  # List of all individual feature lines
    unique_feature_set = set()  # Set of unique feature lines

    with open(input_file, newline='', encoding='utf-8') as csvfile:
        reader = csv.DictReader(csvfile)
        for row in reader:
            status = row.get("Status", "").strip().lower()
            content = row.get("unstable features", "").strip()

            if status == "valid":
                valid_count += 1
                if content:
                    valid_with_unstable_count += 1
                    lines = content.splitlines()
                    for line in lines:
                        line = line.strip()
                        if line.startswith("#![feature(") and line.endswith(")]"):
                            inside = line[len("#![feature("):-2]
                            items = [item.strip() for item in inside.split(",")]
                            for item in items:
                                full_feature = f"#![feature({item})]"
                                all_features.append(full_feature)
                                unique_feature_set.add(full_feature)
                        elif line:
                            all_features.append(line)
                            unique_feature_set.add(line)

    # Compute ratio
    ratio = valid_with_unstable_count / valid_count if valid_count else 0

    # Count top features
    counter = Counter(all_features)
    top_5 = counter.most_common(5)


    # Output
    print("Top 5 unstable features:")
    print(f"{'Feature Line':50} {'Count':>5} {'Proportion':>10}")
    print("-" * 70)
    for feature, count in top_5:
        proportion = count / valid_with_unstable_count if valid_with_unstable_count else 0
        print(f"{feature:50} {count:>5} {proportion * 100:9.2f}%")

    print()
    print(f"Number of valid rows with non-empty 'unstable features' (X): {valid_with_unstable_count}")
    print(f"Ratio (X / valid 'Status'): {ratio * 100:.2f}%")


def print_flag(input_file):
    valid_count = 0  # Total number of valid rows
    valid_with_flag_count = 0  # Valid rows with non-empty "command"
    all_flags = []  # List of all individual flags
    unique_flag_set = set()  # Set of unique flags

    with open(input_file, newline='', encoding='utf-8') as csvfile:
        reader = csv.DictReader(csvfile)
        for row in reader:
            status = row.get("Status", "").strip().lower()
            command = row.get("command", "").strip()
            flag_content = row.get("flag", "").strip()

            if status == "valid":
                valid_count += 1
                if command:  # Use "command" as criterion
                    valid_with_flag_count += 1
                    if flag_content:
                        lines = flag_content.splitlines()
                        for line in lines:
                            flag = line.strip()
                            if flag:
                                all_flags.append(flag)
                                unique_flag_set.add(flag)

    # Compute ratio
    ratio = valid_with_flag_count / valid_count if valid_count else 0

    # Count top flags
    counter = Counter(all_flags)
    top_5 = counter.most_common(5)

    # Output
    print("Top 5 flags:")
    print(f"{'Flag':40} {'Count':>5} {'Proportion':>10}")
    print("-" * 60)
    for flag, count in top_5:
        proportion = count / valid_with_flag_count if valid_with_flag_count else 0
        print(f"{flag:40} {count:>5} {proportion * 100:9.2f}%")
    
    print()
    print(f"Number of valid rows with non-empty 'command' (X): {valid_with_flag_count}")
    print(f"Ratio (X / valid 'Status'): {ratio * 100:.2f}%")


def print_trait(input_file):
    valid_count = 0  # Total number of valid rows
    valid_with_trait_count = 0  # Valid rows with non-empty "trait"
    all_traits = []  # List of all individual traits
    unique_trait_set = set()  # Set of unique trait names

    with open(input_file, newline='', encoding='utf-8') as csvfile:
        reader = csv.DictReader(csvfile)
        for row in reader:
            status = row.get("Status", "").strip().lower()
            content = row.get("trait", "").strip()

            if status == "valid":
                valid_count += 1
                if content:
                    valid_with_trait_count += 1
                    lines = content.splitlines()
                    for line in lines:
                        trait = line.strip()
                        if trait:
                            # Normalize ?Sized to Sized
                            if trait.startswith("?"):
                                trait = trait[1:]
                            all_traits.append(trait)
                            unique_trait_set.add(trait)

    # Compute ratio
    ratio = valid_with_trait_count / valid_count if valid_count else 0

    # Count top traits
    counter = Counter(all_traits)
    top_5 = counter.most_common(5)

    # Output
    print("Top 5 traits:")
    print(f"{'Trait':30} {'Count':>5} {'Proportion':>10}")
    print("-" * 50)
    for trait, count in top_5:
        proportion = count / valid_with_trait_count if valid_with_trait_count else 0
        print(f"{trait:30} {count:>5} {proportion * 100:9.2f}%")
    
    print()
    print(f"Number of valid rows with non-empty 'trait' (X): {valid_with_trait_count}")
    print(f"Ratio (X / valid 'Status'): {ratio * 100:.2f}%")


def print_others(input_file):
    target_columns = ["Lifetime", "std", "dyn", "async", "core"]
    counters = {col: 0 for col in target_columns}
    valid_count = 0

    with open(input_file, newline='', encoding='utf-8') as csvfile:
        reader = csv.DictReader(csvfile)
        for row in reader:
            status = row.get("Status", "").strip().lower()
            if status == "valid":
                valid_count += 1
                for col in target_columns:
                    if row.get(col, "").strip() == "1":
                        counters[col] += 1

    # Print results
    print(f"{'Feature':10} {'Count':>6} {'Frequency':>10}")
    print("-" * 30)
    for col in target_columns:
        freq = counters[col] / valid_count if valid_count else 0
        print(f"{col:10} {counters[col]:>6} {freq * 100:9.2f}%")


if __name__ == "__main__":
    print_unstable_features("./all_issues.csv")
    print()

    print_flag("./all_issues.csv")
    print()

    print_trait("./all_issues.csv")
    print()

    print_others("./all_issues.csv")
    print()