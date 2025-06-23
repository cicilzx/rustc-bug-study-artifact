import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns

if __name__ == '__main__':
    # Set seaborn theme
    sns.set_theme(style="darkgrid")
    plt.tick_params(labelsize=22)
    plt.rcParams['pdf.use14corefonts'] = True

    # Data
    bug_types = ['General Errors', 'MIR Optimization Errors', 'Ownership & Lifetime Errors', 'Type System Errors']
    total_bugs = [123, 46, 41, 91]
    tools = ['ICEMaker', 'Fuzz-rustc', 'Rustlantis', 'Rust team', 'Rust users']
    bug_counts = {
        'Type System Errors': [11, 2, 0, 31, 47],
        'Ownership & Lifetime Errors': [1, 1, 1, 19, 19],
        'MIR Optimization Errors': [8, 0, 5, 14, 19],
        'General Errors': [39, 1, 3, 22, 58]
    }

    colors = sns.color_palette()
    custom_colors = ['#f99655', '#eb6046', '#57b1ab', '#26557b', '#397db7']

    # Create figure and axis
    fig, ax = plt.subplots(figsize=(10, 2.5))

    # Y-axis positions
    y_pos = np.arange(len(bug_types))

    # Draw stacked horizontal bars
    bottoms = np.zeros(len(bug_types))
    bars = []

    for i, tool in enumerate(tools):
        counts = [bug_counts[bug_type][i] for bug_type in bug_types]
        bars.append(ax.barh(y_pos, counts, left=bottoms, height=0.3, label=tool, color=custom_colors[i]))
        bottoms += counts

    # Remove 'developers' from legend if exists
    handles, labels = ax.get_legend_handles_labels()
    handles = [handle for handle, label in zip(handles, labels) if label != 'developers']
    labels = [label for label in labels if label != 'developers']

    # Display total count at the end of each row
    for i, total in enumerate(total_bugs):
        ax.text(total + 1, i, str(total), va='center', ha='left', fontsize=14, color='black')

    # Add custom legend
    ax.legend(handles, labels)

    # Set axis labels and ticks
    ax.set_yticks(y_pos)
    ax.set_yticklabels(bug_types, fontsize=14)
    ax.tick_params(axis='x', labelsize=14)

    # Adjust layout
    plt.tight_layout()

    # Save figure
    plt.savefig(f"Fig10b.pdf", dpi=200, bbox_inches='tight')
