import pandas as pd
import matplotlib.pyplot as plot
import sys


def create_epoch_graph(data_path: str, figure_path: str):
    data = pd.read_csv(data_path)
    xs, ys = data.iloc[:, 0], data.iloc[:, 1]

    plot.xlabel("Epoch")
    plot.ylabel("Mean square error")
    plot.plot(xs, ys, label="MSE")
    plot.legend()
    plot.savefig(figure_path)


if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("usage: command data_path figure_path")
    else:
        data_path = sys.argv[1]
        figure_path = sys.argv[2]
        create_epoch_graph(data_path, figure_path)
