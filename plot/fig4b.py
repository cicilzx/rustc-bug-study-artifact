import matplotlib.pyplot as plt
import seaborn as sns
import numpy as np

if __name__ == '__main__':
    # Set seaborn theme
    sns.set_theme(style="darkgrid")
    plt.tick_params(labelsize=12)
    plt.rcParams['pdf.use14corefonts'] = True

    palette = sns.color_palette()
    colors = ['#26557b', '#397db7', '#57b1ab', '#8ecfb0', '#d5ecbb', '#fed993', '#f99655', '#eb6046', '#ca324c']
    custom_colors = [colors[1], colors[0], colors[6], colors[2]]

    categories = ['MIR Report', 'Dataflow Analysis', 'Borrow Checking', 'MIR Transformation',
                  'HIR Report', 'Type Inference', 'Trait Solving', 'Type & WF Checking']
    values = [17, 9, 27, 53, 19, 10, 37, 69]

    causes = ['Type System Errors', 'Ownership & Lifetime Errors', 'MIR Optimization Errors', 'General Errors']

    bug_counts = {
        'MIR Report': [0, 5, 0, 12],
        'Dataflow Analysis': [0, 0, 0, 9],
        'Borrow Checking': [17, 5, 1, 4],
        'MIR Transformation': [2, 5, 45, 1],
        'HIR Report': [1, 3, 0, 15],
        'Type Inference': [2, 6, 0, 2],
        'Trait Solving': [24, 2, 0, 11],
        'Type & WF Checking': [34, 4, 0, 31]
    }

    # Create figure and axis
    fig, ax = plt.subplots(figsize=(9, 4))

    # Manual y positions
    y_pos = np.arange(len(categories))

    # Add vertical spacing between MIR and HIR bars
    y_shifted = np.array([i if i < 4 else i + 0.5 for i in y_pos])

    # Draw stacked horizontal bars
    bottoms = np.zeros(len(categories))
    bars = []

    for i, tool in enumerate(causes):
        counts = [bug_counts[stage][i] for stage in categories]
        bars.append(ax.barh(y_shifted, counts, left=bottoms, height=0.3, label=tool, color=custom_colors[i]))
        bottoms += counts

    # Show total value and percentage at the end of each bar
    for i, value in enumerate(values):
        if i < 4:
            total = 106
            percent = (value / total) * 100
            plt.text(value + 1, y_shifted[i], f'{value} ({percent:.1f}%)', va='center', fontsize=12)
        else:
            total = 135
            percent = (value / total) * 100
            plt.text(value + 1, y_shifted[i], f'{value} ({percent:.1f}%)', va='center', fontsize=12)

    # Add dashed line between MIR and HIR sections
    plt.axhline(y=y_shifted[3] + 0.75, color='black', linestyle='--', linewidth=1.25)

    # Add 'MIR' label beside first 4 bars
    plt.text(max(values) + 7, np.mean(y_shifted[:4]) + 1, 'MIR', ha='right', va='center', fontsize=17)

    # Add 'HIR' label beside last 4 bars
    plt.text(max(values) + 7, np.mean(y_shifted[4:]) - 0.5, 'HIR', ha='right', va='center', fontsize=17)

    # Set y-axis labels
    plt.yticks(y_shifted, categories)

    # Set x-axis range
    plt.xlim(0, 82)

    # Legend
    handles, labels = ax.get_legend_handles_labels()
    ax.legend(handles, labels)

    # Adjust layout
    plt.tight_layout()

    # Save figure
    plt.savefig(f"./plot/Fig4b.pdf", dpi=200, bbox_inches='tight')
