import numpy
import matplotlib.pyplot as plt


def mcol(v):
    """Reshape a 1-D array into a column vector."""
    return v.reshape((v.shape[0], 1))


def load(fname):
    """Load the dataset from a CSV file and return data matrix and labels."""
    DList = []  # List to store feature vectors
    labelsList = []  # List to store class labels

    # Mapping of class names to numerical labels
    hLabels = {
        'Iris-setosa': 0,
        'Iris-versicolor': 1,
        'Iris-virginica': 2
    }

    with open(fname) as f:
        for line in f:
            try:
                # Extract features and convert them to float
                attrs = line.split(',')[0:-1]
                attrs = mcol(numpy.array([float(i) for i in attrs]))

                # Extract class name and map to numerical label
                name = line.split(',')[-1].strip()
                label = hLabels[name]

                # Append data and labels to lists
                DList.append(attrs)
                labelsList.append(label)
            except:
                pass  # Skip any malformed lines

    # vertical rows stack together horizontally, numpy array of labelList to access numpy functions
    return numpy.hstack(DList), numpy.array(labelsList, dtype=numpy.int32)


def plot_hist(D, L):
    """Plot histograms for each feature, grouped by class labels."""
    D0 = D[:, L == 0]  # Data for Setosa
    D1 = D[:, L == 1]  # Data for Versicolor
    D2 = D[:, L == 2]  # Data for Virginica

    # Feature names
    hFea = {
        0: 'Sepal length',
        1: 'Sepal width',
        2: 'Petal length',
        3: 'Petal width'
    }

    for dIdx in range(4):
        plt.figure()
        plt.xlabel(hFea[dIdx])
        plt.ylabel('Density')

        # Plot histogram for each class
        plt.hist(D0[dIdx, :], bins=10, density=True, alpha=0.4, label='Setosa')
        plt.hist(D1[dIdx, :], bins=10, density=True, alpha=0.4, label='Versicolor')
        plt.hist(D2[dIdx, :], bins=10, density=True, alpha=0.4, label='Virginica')

        plt.legend()
        plt.tight_layout()  # Adjust layout to prevent label cutoff
    plt.show()


def plot_scatter(D, L):
    """Plot scatter plots for all pairs of features, grouped by class labels."""
    D0 = D[:, L == 0]  # Data for Setosa
    D1 = D[:, L == 1]  # Data for Versicolor
    D2 = D[:, L == 2]  # Data for Virginica

    # Feature names
    hFea = {
        0: 'Sepal length',
        1: 'Sepal width',
        2: 'Petal length',
        3: 'Petal width'
    }

    for dIdx1 in range(4):
        for dIdx2 in range(4):
            if dIdx1 == dIdx2:
                continue  # Skip diagonal (same feature)
            plt.figure()
            plt.xlabel(hFea[dIdx1])
            plt.ylabel(hFea[dIdx2])

            # Scatter plot for each class
            plt.scatter(D0[dIdx1, :], D0[dIdx2, :], label='Setosa')
            plt.scatter(D1[dIdx1, :], D1[dIdx2, :], label='Versicolor')
            plt.scatter(D2[dIdx1, :], D2[dIdx2, :], label='Virginica')

            plt.legend()
            plt.tight_layout()  # Adjust layout to prevent label cutoff
        plt.show()


if __name__ == '__main__':
    # Change default font size for plots
    plt.rc('font', size=16)
    plt.rc('xtick', labelsize=16)
    plt.rc('ytick', labelsize=16)

    # Load dataset
    D, L = load('iris.csv')

    # Plot histograms and scatter plots
    # plot_hist(D, L)
    plot_scatter(D, L)

    # Compute mean vector
    # mean restituisce un vettore riga delle medie calcolate. 0 indica le righe, 1 le colonne.
    # Attenzione: scrivere mean(0) non significa dammi la media per righe, bensì per colonne. L'operatore mean calcola
    #             la media lungo l'asse specificato righe(colonne) ovvero per ogni colonna(riga).
    #             axis=0 → media calcolata lungo l'asse 0, cioè sulle righe, restituendo un valore per ogni colonna.
    #             axis=1 → media calcolata lungo l'asse 1, cioè sulle colonne, restituendo un valore per ogni riga.

    # REGOLA PRATICA: IL VALORE SPECIFICATO COLLASSA, OSSIA DA n DI QUESTO A 1.
    #                 Esempio: qui vogliamo passare da 4x150 a 4x1 ossia devo far collassare le colonne -> mean(1)
    mu = D.mean(1).reshape((D.shape[0], 1))
    print('Mean:')
    print(mu)
    print()

    # Center the data by subtracting the mean
    DC = D - mu

    # plot of the centered dataframe
    # plot_hist(DC, L)
    plot_scatter(DC, L)

    # Compute covariance matrix
    C = ((D - mu) @ (D - mu).T) / float(D.shape[1])
    print('Covariance matrix:')
    print(C)
    print()

    # Compute variance and standard deviation
    # come per la media devi indicare cosa far collassare, collassando le colonne avrò 4x1
    # e i metodi numpy ritornano vettori riga
    var = D.var(1)
    std = D.std(1)
    print('Variance:', var)
    print('Std. dev.:', std)
    print()

    # Compute statistics for each class(flowers) separately
    for cls in [0, 1, 2]:
        print('Class', cls)
        DCls = D[:, L == cls]  # Extract class-specific data

        # Compute mean vector for the class
        mu = DCls.mean(1).reshape(DCls.shape[0], 1)
        print('Mean:')
        print(mu)

        # Compute covariance matrix for the class
        C = ((DCls - mu) @ (DCls - mu).T) / float(DCls.shape[1])
        print('Covariance:')
        print(C)

        # Compute variance and standard deviation for the class
        var = DCls.var(1)
        std = DCls.std(1)
        print('Variance:', var)
        print('Std. dev.:', std)
        print()
