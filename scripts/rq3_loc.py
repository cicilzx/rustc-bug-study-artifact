import pandas as pd

def count_lines_in_series(series):
    return series.apply(lambda x: x.count('\n') + 1 if pd.notna(x) and x.strip() != '' else 0)

def print_stats(name, line_counts):
    if line_counts.empty:
        print(f"No data for {name}")
        return
    
    non_zero = line_counts[line_counts > 0]
    
    col1_width = 26
    col2_width = 12
    
    print(f"{name.ljust(col1_width)}{'Value'.rjust(col2_width)}")
    print("-" * (col1_width + col2_width))
    print(f"{'Mean'.ljust(col1_width)}{line_counts.mean():>{col2_width}.2f}")
    print(f"{'Median'.ljust(col1_width)}{line_counts.median():>{col2_width}.2f}")
    
    
    if len(non_zero.unique()) > 1:
        second_min = sorted(non_zero.unique())[1]
        print(f"{'Second smallest non-zero'.ljust(col1_width)}{second_min:>{col2_width}}")
    else:
        print(f"{'Second smallest non-zero'.ljust(col1_width)}{'N/A':>{col2_width}}")
    print(f"{'Max'.ljust(col1_width)}{line_counts.max():>{col2_width}}")
    print()

if __name__ == '__main__':
    df = pd.read_csv('./data/all_issues.csv')
    valid_df = df[df['Status'] == 'valid']
    
    original_counts = count_lines_in_series(valid_df['Test case (original)'])
    reduced_counts = count_lines_in_series(valid_df['Test case (reduced)'])
    
    print_stats("Test case (original)", original_counts)
    print_stats("Test case (reduced)", reduced_counts)
