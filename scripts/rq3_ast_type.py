import csv
from collections import defaultdict

# 设置 CSV 文件路径
csv_file_path = "./type_node_counts.csv"

# 存储每个节点类型的统计信息
node_stats = defaultdict(lambda: {
    "total": 0,
    "file_count": 0,
    "max_count": 0
})

total_file_count = 0

# 读取并处理 CSV
with open(csv_file_path, newline='') as csvfile:
    reader = csv.DictReader(csvfile)
    node_types = reader.fieldnames[1:]  # 排除 'File' 列

    for row in reader:
        total_file_count += 1
        for node in node_types:
            count = int(row[node])
            node_stats[node]["total"] += count
            if count > 0:
                node_stats[node]["file_count"] += 1
                node_stats[node]["max_count"] = max(node_stats[node]["max_count"], count)

# 构造统计结果
results = []
for node in node_types:
    total = node_stats[node]["total"]
    files = node_stats[node]["file_count"]
    average = total / files if files > 0 else 0
    max_count = node_stats[node]["max_count"]
    prevalence = files / total_file_count if total_file_count > 0 else 0
    results.append((node, total, prevalence, average, max_count))

# 按 Prevalence 从高到低排序
results.sort(key=lambda x: x[2], reverse=True)

# 输出结果
print(f"{'Node Type':<15} {'Total':>10} {'Prevalence':>12} {'Average':>10} {'Max':>10}")
for node, total, prevalence, average, max_count in results:
    print(f"{node:<15} {total:>10} {prevalence:>12.2%} {average:>10.2f} {max_count:>10}")
