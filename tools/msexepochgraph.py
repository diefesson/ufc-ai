import pandas as pd
import matplotlib.pyplot as plot

_DATA_PATH = "output/output-1.csv"
_OUTPUT_PATH = "output/output-1.png"


def main():
    data = pd.read_csv(_DATA_PATH)
    xs, ys = data.iloc[:, 0], data.iloc[:, 1]

    plot.xlabel("Epoch")
    plot.ylabel("Mean square error")
    plot.plot(xs, ys)
    plot.legend()
    plot.savefig(_OUTPUT_PATH)


if __name__ == "__main__":
    main()
