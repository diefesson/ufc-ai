import pandas as pd
import matplotlib.pyplot as plot
import sys


def create_epoch_graph(
    data_path: str,
    figure_path: str,
    x_label="Epoch",
    y_label="Means square error",
    line_label="MSE",
):
    data = pd.read_csv(data_path, header=None)
    xs, ys = data.iloc[:, 0], data.iloc[:, 1]

    plot.xlabel(x_label)
    plot.ylabel(y_label)
    plot.plot(xs, ys, label=line_label)
    plot.legend()
    plot.savefig(figure_path)


if __name__ == "__main__":
    argc = len(sys.argv)
    if not (3 <= argc <= 6):
        print(
            "usage: command <data_path> <figure_path> [x_label] [y_label] [line_label]"
        )
    else:
        data_path = sys.argv[1]
        figure_path = sys.argv[2]
        create_epoch_graph(data_path, figure_path)
