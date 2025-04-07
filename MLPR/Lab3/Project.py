import matplotlib.pyplot as plt
import numpy
import scipy

import Optimize_threshold as opt


def plot_hist(D, L):
    Dtrue = D[:, L == 1]
    Dfalse = D[:, L == 0]

    for idFeature in range(D.shape[0]):
        plt.figure()
        string = f"feature {idFeature+1}"
        plt.xlabel(string)
        plt.ylabel("density") # dove si concentra il maggior numero dei dati

        # only the row(feature) intended
        plt.hist(Dfalse[idFeature, :], bins=50, density=True, alpha=0.4, label='False Fingerprint')
        plt.hist(Dtrue[idFeature, :], bins=50, density=True, alpha=0.4, label='True Fingerprint')

        plt.legend()
        plt.tight_layout()  # Adjust layout to prevent label cutoff
    plt.show()

def load_fingerprint():
    L = []  # label: true finger, false finger
    D = []  # dataset: 6 dimension features
    with open("trainData.txt", "r") as f:
        for line in f.readlines():
            row = [column.strip() for column in line.split(",")]

            label = int(row[-1])
            features = numpy.array([float(feature) for feature in row[:-1]])

            L.append(label)
            D.append(features.reshape(features.shape[0], 1))

        #  righe da attaccare una di fianco all'altra -> horizontal stack
        D = numpy.hstack(D)
        # also L need to become a numpy object
        L = numpy.array(L, dtype=numpy.int32)

        return D, L


def vcol(row):
    return row.reshape((row.shape[0], 1))


def compute_mean_covariance(D):
    # Compute mean vector
    mu = vcol(D.mean(1))
    # Center the data by subtracting the mean
    Dc = D - mu
    # Compute covariance matrix
    C = (Dc @ Dc.T) / float(Dc.shape[1])
    print(f"Mean:\n{mu}\nCovariance matrix:\n{C}\n")

    return mu, C


def project_dataset(W, D):
    return W.T @ D


def compute_pca(D, m):
    mu, C = compute_mean_covariance(D)

    # Once we have computed the data covariance matrix, we need to compute its eigenvectors and eigenvalues.
    # - For a generic square matrix we can use the library function numpy.linalg.eig
    # - if the covariance matrix is symmetric, we can use the more specific function numpy.linalg.eigh
    # In both cases the functions returns the eigenvalues (s), and the corresponding eigenvectors (columns of U).
    print(C.shape)
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

    # projection of the dataset
    D_pca = project_dataset(P, D)

    return D_pca


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


def compute_sw_sb(D, L):
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
    overall_mean = vcol(D.mean(axis=1))

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
        class_mean, class_cov = compute_mean_covariance(class_samples)

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


def compute_lda(D, L):
    Sb, Sw = compute_sw_sb(D, L)
    m = D.shape[0] - 1  # max val number of class - 1
    W = compute_eigenvalue(Sb, Sw, m)

    # Projecting the data
    D_lda = project_dataset(W, D)

    return D_lda

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


def classification_with_lda(D, L):
    # DTR and LTR are model training data and labels
    # DVAL and LVAL are validation data and labels
    (DTR, LTR), (DVAL, LVAL) = split_db_2to1(D, L)

    # Compute lda
    Sb, Sw = compute_sw_sb(DTR, LTR)  # b: between class variation \ w: within class variation
    m = DTR.shape[0] - 1  # max val number of class - 1
    W = compute_eigenvalue(Sb, Sw, m)

    # Projecting the data
    DTR_lda = project_dataset(W, DTR)
    DVAL_lda = project_dataset(W, DVAL)

    # Plotting
    #plot_hist(DTR_lda, LTR)
    #plot_hist(DVAL_lda, LVAL)

    # Classified the projected data
    mu_false_fingerprint = DTR_lda[0, LTR == 0].mean()
    mu_true_fingerprint = DTR_lda[0, LTR == 1].mean()
    threshold = (mu_true_fingerprint + mu_false_fingerprint) / 2.0  # Projected samples have only 1 dimension
    PVAL = numpy.zeros(shape=LVAL.shape, dtype=numpy.int32)
    PVAL[DVAL_lda[0] >= threshold] = 1 # predicted as true
    PVAL[DVAL_lda[0] < threshold] = 0 # predicted as false

    # validation
    num_different = numpy.sum(LVAL != PVAL)  # != between numpy vectors return a list of boolean (0 or 1)
    print(f"Threshold 𝒕 {threshold}\nNumber of different elements: {num_different} out of {len(LVAL)}\nError rate: {100*num_different/len(LVAL)}%")
    print(f"{LVAL}\n{PVAL}")


    # AI generated to Minimize the Error rate
    best_threshold, best_acc, metrics = opt.optimize_threshold(DTR_lda, LTR, DVAL_lda, LVAL,
                                                           metric='balanced_accuracy',
                                                           return_metrics=True) # Per visualizzare il grafico delle metriche al variare della soglia
    opt.plot_threshold_metrics(metrics)


def prepocessing_with_pca_classification_with_lda(D, L, m):
    # DTR and LTR are model training data and labels
    # DVAL and LVAL are validation data and labels
    (DTR, LTR), (DVAL, LVAL) = split_db_2to1(D, L)

    # Compute pca
    mu, C = compute_mean_covariance(DTR)

    # NOTE: Since the covariance matrix is semi-definite positive
    #       we can also get the sorted eigenvectors from the Singular Value Decomposition (svd)
    U, s, Vh = numpy.linalg.svd(C)
    # In this case, the singular values (which are equal to the eigenvalues) are sorted in descending order,
    # and the columns of U are the corresponding eigenvectors
    P = U[:, 0:m]

    # Projecting the data
    DTR_pca = project_dataset(P, DTR)
    DVAL_pca = project_dataset(P, DVAL)

    # Compute lda
    Sb, Sw = compute_sw_sb(DTR_pca, LTR)  # b: between class variation \ w: within class variation
    m = DTR.shape[0] - 1  # max val number of class - 1
    W = compute_eigenvalue(Sb, Sw, m)

    # Projecting the data
    DTR_lda = project_dataset(W, DTR_pca)
    DVAL_lda = project_dataset(W, DVAL_pca)

    # Classified the projected data
    mu_false_fingerprint = DTR_lda[0, LTR == 0].mean()
    mu_true_fingerprint = DTR_lda[0, LTR == 1].mean()
    threshold = (mu_true_fingerprint + mu_false_fingerprint) / 2.0  # Projected samples have only 1 dimension
    PVAL = numpy.zeros(shape=LVAL.shape, dtype=numpy.int32)
    PVAL[DVAL_lda[0] >= threshold] = 1 # predicted as true
    PVAL[DVAL_lda[0] < threshold] = 0 # predicted as false

    # validation
    num_different = numpy.sum(LVAL != PVAL)  # != between numpy vectors return a list of boolean (0 or 1)
    print(f"Threshold 𝒕 {threshold}\nNumber of different elements: {num_different} out of {len(LVAL)}\nError rate: {100 * num_different / len(LVAL)}%")
    print(f"{LVAL}\n{PVAL}")


if __name__ == '__main__':
    # load dataset of true/false fingerprint
    D, L = load_fingerprint()

    # plot with pca applied
    # m = 6
    # D_pca = compute_pca(D, m)
    # plot_hist(D_pca, L)

    # plot with lda applied
    # since we have just two classes lda return a single direction
    # D_lda = compute_lda(D, L)
    # plot_hist(D_lda, L)

    # LDA for classification
    # classification_with_lda(D, L)

    # PCA as dimensionality reduction and LDA for classification
    m = 1
    prepocessing_with_pca_classification_with_lda(D, L, m)



