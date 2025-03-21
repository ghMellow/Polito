from statistics import variance

import matplotlib.pyplot as plt
import numpy


def plotHist(D, L):
    Dtrue = D[:, L == 1]
    Dfalse = D[:, L == 0]

    for idFeature in range(6):
        plt.figure()
        string = f"feature {idFeature+1}"
        plt.xlabel(string)
        plt.ylabel("density") # dove si concentra il maggior numero dei dati

        # only the row(feature) intended
        plt.hist(Dtrue[idFeature, :], bins=10, density=True, alpha=0.4, label='True')
        plt.hist(Dfalse[idFeature, :], bins=10, density=True, alpha=0.4, label='False')

        plt.legend()
        plt.tight_layout()  # Adjust layout to prevent label cutoff
    plt.show()


def plot_scatter(D, L):
    Dtrue = D[:, L == 1]
    Dfalse = D[:, L == 0]

    for dIdx1 in range(6):
        for dIdx2 in range(6):
            if dIdx1 == dIdx2:
                continue  # Skip diagonal (same feature)
            plt.figure()
            str1 = f"feature {dIdx1+1}"
            str2 = f"feature {dIdx2+1}"
            plt.xlabel(str1)
            plt.ylabel(str2)

            # Scatter plot for each class
            plt.scatter(Dtrue[dIdx1, :], Dtrue[dIdx2, :], label='True')
            plt.scatter(Dfalse[dIdx1, :], Dfalse[dIdx2, :], label='False')

            plt.legend()
            plt.tight_layout()  # Adjust layout to prevent label cutoff
        plt.show()

if __name__ == '__main__':

    L = [] # label: true finger, false finger
    D = [] # dataset: 6 dimension features
    with open("trainData.txt", "r") as f:
        for line in f.readlines():
            row = [column.strip() for column in line.split(",")]

            label = int(row[-1])
            features = numpy.array( [float(feature) for feature in row[:-1]] )

            L.append(label)
            D.append(features.reshape(features.shape[0], 1))

        #  righe da attaccare una di fianco all'altra -> horizontal stack
        D = numpy.hstack(D)
        # also L need to become a numpy object
        L = numpy.array(L, dtype=numpy.int32)

    plotHist(D, L)
    plot_scatter(D, L)

    # Dataset mean
    mu = D.mean(axis=1).reshape(D.shape[0], 1)
    print(f"Dataset mean:\n{mu}\n") # mean is close to 0 for each feature so D and DC are similar/equal.

    # Centered dataset
    DC = D - mu
    print(f"Centered dataset:\n{DC}\n")
    plotHist(D, L)
    plot_scatter(D, L)

    # Covariance Matrix
    C = ((D - mu) @ (D - mu).T) / float(D.shape[1])
    # grom this we can obtain the variance (values on the principal diagonal) and standard deviation
    # or we can simply use numpy (1 to collapse columns)
    variance = DC.var(axis=1)
    standard_deviation = DC.std(axis=1)
    print(f"Covariance Matrix: \n{C}\n Variance: {variance}\n Standard Deviation: {standard_deviation}\n")