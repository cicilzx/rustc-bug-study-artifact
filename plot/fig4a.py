import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns

if __name__ == '__main__':
    # Set seaborn theme
    sns.set_theme(style="darkgrid")
    plt.tick_params(labelsize=22)
    plt.rcParams['pdf.fonttype'] = 42
    # plt.rcParams['pdf.use14corefonts'] = True

    # Data
    stages = ['6. LLVM', '5. Utility', '4. Code Gen', '3. MIR', '2. HIR', '1. AST']
    total_bugs = [7, 24, 5, 106, 135, 24]
    causes = ['Type System Errors', 'Ownership & Lifetime Errors', 'MIR Optimization Errors', 'General Errors']
    bug_counts = {
        '6. LLVM': [0, 0, 0, 7],
        '5. Utility': [10, 0, 0, 14],
        '4. Code Gen': [0, 0, 0, 5],
        '3. MIR': [19, 15, 46, 26],
        '2. HIR': [61, 15, 0, 59],
        '1. AST': [1, 11, 0, 12]
    }

    colors = ['#26557b', '#397db7', '#57b1ab', '#8ecfb0', '#d5ecbb', '#fed993', '#f99655', '#eb6046', '#ca324c']
    custom_colors = [colors[1], colors[0], colors[6], colors[2]]

    # Create figure and axis
    fig, ax = plt.subplots(figsize=(8, 4))

    # Y-axis positions
    y_pos = np.arange(len(stages))

    # Draw stacked horizontal bars
    bottoms = np.zeros(len(stages))
    bars = []

    for i, tool in enumerate(causes):
        counts = [bug_counts[stage][i] for stage in stages]
        bars.append(ax.barh(y_pos, counts, left=bottoms, height=0.3, label=tool, color=custom_colors[i]))
        bottoms += counts

    # Remove 'developers' from legend if present
    handles, labels = ax.get_legend_handles_labels()
    handles = [handle for handle, label in zip(handles, labels) if label != 'developers']
    labels = [label for label in labels if label != 'developers']

    # Display total count and percentage at the end of each bar
    for i, total in enumerate(total_bugs):
        percent = (total / 301) * 100
        ax.text(total + 1, i, f'{total} ({percent:.1f}%)', va='center', ha='left', fontsize=14, color='black')

    # Add custom legend
    ax.legend(handles, labels)

    # Set y-axis labels
    ax.set_yticks(y_pos)
    ax.set_yticklabels(stages)

    plt.xlim(0, 165)

    plt.tight_layout()
    plt.savefig(f"./plot/Fig4a.pdf", dpi=200, bbox_inches='tight')
