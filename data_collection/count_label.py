import csv
from collections import Counter
import ast



filename = 'issues.csv'
column_index = 1


all_elements = []


with open(filename, newline='', encoding='utf-8') as csvfile:
    csvreader = csv.reader(csvfile)
    

    next(csvreader)
    
    for row in csvreader:
        if row:
            try:
                element_set = ast.literal_eval(row[column_index])
                if isinstance(element_set, set):
                    all_elements.extend(element_set)
            except (ValueError, SyntaxError):
                continue

counter = Counter(all_elements)

for element, count in counter.items():
    print(f'{element}: {count}')

