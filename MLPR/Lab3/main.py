import numpy
from matplotlib import pyplot as plt
import scipy


def mcol(row):
    return row.reshape((row.shape[0], 1))


def load(fname):
    """Load the dataset from file and return data matrix and labels."""
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
                attrs = numpy.array([float(i) for i in attrs])
                attrs = mcol(attrs)

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


def compute_mean_covarianceMatrix(D):
    # Compute mean vector
    mu = mcol(D.mean(1))
    # Center the data by subtracting the mean
    Dc = D - mu
    # Compute covariance matrix
    C = (Dc @ Dc.T) / float(Dc.shape[1])
    print(f"Mean:\n{mu}\nCovariance matrix:\n{C}\n")

    return mu, C


def compute_eigenvalues_eigenvectors(C, m):
    if m < 2:
        m = 2  # min 2 columns since we are plotting the 2 columns with the higher value of variance

    # Once we have computed the data covariance matrix, we need to compute its eigenvectors and eigenvalues.
    # - For a generic square matrix we can use the library function numpy.linalg.eig
    # - if the covariance matrix is symmetric, we can use the more specific function numpy.linalg.eigh
    # In both cases the functions returns the eigenvalues (s), and the corresponding eigenvectors (columns of U).
    if C.shape[0] != C.shape[1]:
        s, U = numpy.linalg.eig(C)
    else:
        s, U = numpy.linalg.eigh(C)

    # now we need to sort the eigenvalues sorted from smallest to largest
    # The m leading eigenvectors can be retrieved from U (here we also reverse the order of the columns of U
    # so that the leading eigenvectors are in the first m columns)
    P = U[:, ::-1][:, 0:m]

    # NOTE: Since the covariance matrix is semi-definite positive
    #       we can also get the sorted eigenvectors from the Singular Value Decomposition (svd)
    U, s, Vh = numpy.linalg.svd(C)
    # In this case, the singular values (which are equal to the eigenvalues) are sorted in descending order,
    # and the columns of U are the corresponding eigenvectors
    P = U[:, 0:m]

    return P


def compute_pca(C):
    m = C.shape[0]  # m determines the subspace, in this case we are not doing any reduction
    P = compute_eigenvalues_eigenvectors(C, m) # solo quando è diagonale ho gli autovalori che rappresentano di ogni feature 

    # Finally, we can apply the projection to a single point x or to a matrix of samples D as
    # x = D[0][0] # e.g. of a point
    # y = numpy.dot(P.T, x)
    # or
    Dpca = numpy.dot(P.T, D)

    return Dpca


def compute_generalized_eigenvalue(SB, SW, m):
    # Note: the numpy.linalg.eigh function cannot be used as it does not solve the generalized problem
    # thus we can use the scipy.linalg.eigh function (you need to import scipy.linalg), which solves the generalized
    # eigenvalue problem for hermitian (including real symmetric) matrices
    s, U = scipy.linalg.eigh(SB, SW)
    W = U[:, ::-1][:, 0:m]

    # Notice that the columns of W are not necessarily orthogonal. If we want, we can find a basis U for the
    # subspace spanned by W using the singular value decomposition of W:
    """if not numpy.allclose(W.T @ W, numpy.eye(W.shape[1]), atol=1e-6):
        UW, _, _ = numpy.linalg.svd(W)
        W = UW[:, 0:m]"""

    return W


def compute_lda(D, L):
    """
    Compute the within-class (Sw) and between-class (Sb) covariance matrices.

    Parameters:
        D: numpy array of shape (n_features, n_samples)
        L: numpy array of shape (n_samples,), containing class labels

    Returns:
        Sw: Within-class scatter matrix
        Sb: Between-class scatter matrix
    """
    n_features = D.shape[0]
    N = D.shape[1]  # number of elements of the dataset
    classes = numpy.unique(L)
    overall_mean = mcol(D.mean(1))

    Sw = numpy.zeros((n_features, n_features))
    Sb = numpy.zeros((n_features, n_features))

    for c in classes:
        D_class = D[:, L == c]
        nc = D_class.shape[1]  # weigh, number of elements of the class
        mu_c, C_c = compute_mean_covarianceMatrix(D_class)

        # Compute within-class covariance
        Sw += nc * C_c

        # Compute between-class covariance
        mean_diff = mu_c - overall_mean
        Sb += nc * (mean_diff @ mean_diff.T)

    Sb = Sb / N
    Sw = Sw / N
    print(f"Sb:\n{Sb}\nSw:\n{Sw}")

    m = n_features
    W = compute_generalized_eigenvalue(Sb, Sw, m)

    USol = numpy.load('IRIS_LDA_matrix_m2.npy')  # May have different signs for different directions
    print(USol)
    print(numpy.linalg.svd(numpy.hstack([W, USol]))[1])

    return W.T @ D


def plot_scatter(D, L, invert_x_axes, invert_y_axes):
    """
    Plot scatter plot for PCA-reduced data (2D), grouped by class labels.

    Principal Component Analysis (PCA) is a technique for dimensionality reduction that transforms data into principal
    components (PCs). The first principal component (PC1) captures the largest variance in the data, while the second
    principal component (PC2) captures the second-largest variance, orthogonal to PC1. By projecting data onto these two
    components, it becomes easier to visualize and interpret complex data, reducing it to two dimensions.
    In a scatter plot, PC1 is typically on the x-axis and PC2 on the y-axis, helping to distinguish data points,
    like different classes in a dataset.
    """

    # from dataset to matrices of features separated for each flowers
    D0 = D[:, L == 0]  # Setosa
    D1 = D[:, L == 1]  # Versicolor
    D2 = D[:, L == 2]  # Virginica

    plt.figure(figsize=(8, 6))
    plt.xlabel(
        "First Principal Component")  # The first principal component (PC1) represents the direction of greatest variance in the data.
    plt.ylabel(
        "Second Principal Component")  # The second principal component (PC2) represents the direction of second-highest variance, orthogonal to PC1.

    # Scatter plot for each class
    # Note: axes y *-1 to flip the graph and make it the same as the one shown in the pdf
    plt.scatter(D0[0, :] * invert_x_axes, (D0[1, :] * invert_y_axes), label='Setosa', color='blue')
    plt.scatter(D1[0, :] * invert_x_axes, (D1[1, :] * invert_y_axes), label='Versicolor', color='orange')
    plt.scatter(D2[0, :] * invert_x_axes, (D2[1, :] * invert_y_axes), label='Virginica', color='green')

    plt.legend()
    plt.title("PCA of IRIS Dataset")
    plt.show()


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

    for dIdx in range(D.shape[0]):
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


if __name__ == '__main__':
    fname = "iris.csv"

    D, L = load(fname)
    print(f"{D}\n{L}\n")

    mu, C = compute_mean_covarianceMatrix(D)

    # PCA reduction
    D_pca = compute_pca(C)
    plot_scatter(D_pca, L, 1, -1)
    plot_hist(D_pca, L)

    # LDA reduction
    D_lda = compute_lda(D, L)
    plot_scatter(D_lda, L, 1, 1)
    plot_hist(D_lda, L)
