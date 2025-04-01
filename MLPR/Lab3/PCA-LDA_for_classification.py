import scipy

import main
import numpy
from matplotlib import pyplot as plt


def split_db_2to1(D, L, seed=0):
    nTrain = int(D.shape[1] * 2.0 / 3.0)
    numpy.random.seed(seed)
    idx = numpy.random.permutation(D.shape[1])
    idxTrain = idx[0:nTrain]
    idxTest = idx[nTrain:]
    DTR = D[:, idxTrain]
    DVAL = D[:, idxTest]
    LTR = L[idxTrain]
    LVAL = L[idxTest]
    return (DTR, LTR), (DVAL, LVAL)


def mcol(row):
    return row.reshape((row.shape[0], 1))


def compute_mean_covariancematrix(D):
    # Compute mean vector
    mu = mcol(D.mean(1))
    # Center the data by subtracting the mean
    Dc = D - mu
    # Compute covariance matrix
    C = (Dc @ Dc.T) / float(Dc.shape[1])
    print(f"Mean:\n{mu}\nCovariance matrix:\n{C}\n")

    return mu, C


def compute_eigenvalue(SB, SW, m):
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


def project_dataset(W, D):
    return W.T @ D


def compute_pca(C, m):
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


def compute_lda(D, L):
    """
    Computes the within-class (Sw) and between-class (Sb) covariance matrices for LDA.

    Parameters:
        D: numpy array of shape (n_features, n_samples)
        L: numpy array of shape (n_samples,), containing class labels

    Returns:
        Projection of data in LDA space
    """
    # Extract dimensions
    n_features, n_samples = D.shape

    # Calculate global mean
    overall_mean = mcol(D.mean(axis=1))

    # Initialize covariance matrices
    Sw = numpy.zeros((n_features, n_features))  # Within-class scatter matrix
    Sb = numpy.zeros((n_features, n_features))  # Between-class scatter matrix

    # Process each class
    for class_label in numpy.unique(L):
        # Extract samples for current class
        class_samples = D[:, L == class_label]

        # Number of samples in this class
        n_class_samples = class_samples.shape[1]

        # Compute mean and covariance matrix for the class
        class_mean, class_cov = compute_mean_covariancematrix(class_samples)

        # Update within-class matrix
        Sw += n_class_samples * class_cov

        # Calculate difference between class mean and global mean
        mean_diff = class_mean - overall_mean

        # Update between-class matrix
        Sb += n_class_samples * (mean_diff @ mean_diff.T)

    # Normalize matrices
    Sb = Sb / n_samples
    Sw = Sw / n_samples

    # Print calculated matrices
    print(f"Sb matrix (between-class):\n{Sb}\n")
    print(f"Sw matrix (within-class):\n{Sw}\n")

    return Sb, Sw


def plot_hist(D, L, axes_x):
    """Plot histograms for each feature, grouped by class labels."""
    D1 = D[:, L == 1]  # Data for Versicolor
    D2 = D[:, L == 2]  # Data for Virginica

    # Feature names
    hFea = {
        0: 'Sepal length',
        1: 'Sepal width',
        2: 'Petal length',
        3: 'Petal width'
    }

    for dIdx in range(1):
        plt.figure()
        plt.xlabel(hFea[dIdx])
        plt.ylabel('Density')

        # Plot histogram for each class
        plt.hist(D1[dIdx, :] * axes_x, bins=5, density=True, alpha=0.4, label='Versicolor')
        plt.hist(D2[dIdx, :] * axes_x, bins=5, density=True, alpha=0.4, label='Virginica')

        plt.legend()
        plt.tight_layout()  # Adjust layout to prevent label cutoff
    plt.show()


def classification_with_pca(D, L):
    # DTR and LTR are model training data and labels
    # DVAL and LVAL are validation data and labels
    (DTR, LTR), (DVAL, LVAL) = split_db_2to1(D, L)

    # Compute pca
    mu, C = compute_mean_covariancematrix(DTR)
    m = C.shape[0]
    P = compute_pca(C, m)

    # Projecting the data
    DTR_pca = project_dataset(P, DTR)
    DVAL_pca = project_dataset(P, DVAL)

    # Plotting
    plot_hist(DTR_pca, LTR, -1)
    plot_hist(DVAL_pca, LVAL, -1)

    # Classified the projected data
    threshold = (DTR_pca[0, LTR == 1].mean() + DTR_pca[
        0, LTR == 2].mean()) / 2.0  # Projected samples have only 1 dimension
    PVAL = numpy.zeros(shape=LVAL.shape, dtype=numpy.int32)
    PVAL[DVAL_pca[0] >= threshold] = 2
    PVAL[DVAL_pca[0] < threshold] = 1

    # validation
    num_different = numpy.sum(LVAL != PVAL)  # != between numpy vectors return a list of boolean (0 or 1)
    print(f"Number of different elements: {num_different} out of {len(LVAL)}")
    print(f"{LVAL}\n{PVAL}")


def classification_with_lda(D, L):
    # DTR and LTR are model training data and labels
    # DVAL and LVAL are validation data and labels
    (DTR, LTR), (DVAL, LVAL) = split_db_2to1(D, L)

    # Compute lda
    Sb, Sw = compute_lda(DTR, LTR)  # b: between class variation \ w: within class variation
    m = DTR.shape[0] - 1  # max val number of class - 1
    W = compute_eigenvalue(Sb, Sw, m)

    # Projecting the data
    DTR_lda = project_dataset(W, DTR)
    DVAL_lda = project_dataset(W, DVAL)

    # Plotting
    plot_hist(DTR_lda, LTR, 1)
    plot_hist(DVAL_lda, LVAL, 1)

    # Classified the projected data
    threshold = (DTR_lda[0, LTR == 1].mean() + DTR_lda[
        0, LTR == 2].mean()) / 2.0  # Projected samples have only 1 dimension
    PVAL = numpy.zeros(shape=LVAL.shape, dtype=numpy.int32)
    PVAL[DVAL_lda[0] >= threshold] = 2
    PVAL[DVAL_lda[0] < threshold] = 1

    # validation
    num_different = numpy.sum(LVAL != PVAL)  # != between numpy vectors return a list of boolean (0 or 1)
    print(f"Number of different elements: {num_different} out of {len(LVAL)}")
    print(f"{LVAL}\n{PVAL}")


def prepocessing_with_pca_classification_with_lda(D, L):
    # DTR and LTR are model training data and labels
    # DVAL and LVAL are validation data and labels
    (DTR, LTR), (DVAL, LVAL) = split_db_2to1(D, L)

    # Compute pca
    mu, C = compute_mean_covariancematrix(DTR)
    m = 2
    P = compute_pca(C, m)

    # Projecting the data
    DTR_pca = project_dataset(P, DTR)
    DVAL_pca = project_dataset(P, DVAL)

    # Compute lda
    Sb, Sw = compute_lda(DTR_pca, LTR)  # b: between class variation \ w: within class variation
    m = DTR.shape[0] - 1  # max val number of class - 1
    W = compute_eigenvalue(Sb, Sw, m)

    # Projecting the data
    DTR_lda = project_dataset(W, DTR_pca)
    DVAL_lda = project_dataset(W, DVAL_pca)

    # Plotting
    plot_hist(DTR_lda, LTR, -1)
    plot_hist(DVAL_lda, LVAL, -1)

    # Classified the projected data
    threshold = (DTR_lda[0, LTR == 1].mean() + DTR_lda[0, LTR == 2].mean()) / 2.0  # Projected samples have only
                                                                                   # 1 dimension
    PVAL = numpy.zeros(shape=LVAL.shape, dtype=numpy.int32)
    PVAL[DVAL_lda[0] >= threshold] = 2
    PVAL[DVAL_lda[0] < threshold] = 1

    # validation
    num_different = numpy.sum(LVAL != PVAL)  # != between numpy vectors return a list of boolean (0 or 1)
    print(f"Number of different elements: {num_different} out of {len(LVAL)}")
    print(f"{LVAL}\n{PVAL}")


if __name__ == '__main__':
    fname = "iris.csv"

    # We now turn our attention to applications of PCA and LDA, focusing on a binary classification task.
    DIris, LIris = main.load(fname)
    D = DIris[:, LIris != 0]
    L = LIris[LIris != 0]

    # D and L will contain the samples and labels of classes 1 and 2 (versicolor and virginica).

    # In this section, we want to apply PCA and LDA to the binary classification problem. In particular, we employ
    # LDA to identify the (single, since we have only 2 classes) discriminant direction of the two-class version
    # of the dataset (which, in general, may diﬀer from the discriminant directions of the 3-class problem).
    # We will also employ PCA as a pre-processing for LDA.

    # 1.
    #classification_with_lda(D, L)

    # 2.
    #classification_with_pca(D, L)

    # 3.
    prepocessing_with_pca_classification_with_lda(D, L)
