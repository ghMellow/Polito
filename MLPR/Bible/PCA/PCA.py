import numpy as np

from mean_covariance import compute_mu_C
from linear_projection_classifier import binary_classification_threshold, classify_binary, evaluate_errors


def compute_pca(D, m):
    # Compute mean vector (mu) and covariance matrix (C) of the dataset D
    mu, C = compute_mu_C(D)
    # Perform Singular Value Decomposition (SVD) on the covariance matrix
    # Decompose C with SVD: C = U @ diag(s) @ Vh, U = principal directions
    U, s, Vh = np.linalg.svd(C)
    # Select the first m principal components (the eigenvectors corresponding to the largest singular values)
    P = U[:, 0:m]
    return P, mu


def apply_pca(P, mu, D):
    """
    Apply PCA transformation
    Le componenti principali sono calcolate assumendo dati centrati / sono definite rispetto alla media dei dati.
    Devi centrare perché PCA cerca variazioni rispetto al centro dei dati
    """
    return P.T @ (D - mu)


def apply_pca_reduction(DTR, DVAL, pca_dim):
    P, mu = compute_pca(DTR, m=pca_dim)
    DTR_pca = apply_pca(P, mu, DTR)
    DVAL_pca = apply_pca(P, mu, DVAL)
    print(f"PCA: reduced from {DTR.shape[0]}D to {pca_dim}D")
    return DTR_pca, DVAL_pca


def pca_binary_pipeline(DTR, LTR, DVAL, LVAL, class1=1, class2=2):
    """
    PCA binary classification pipeline (using first principal component)
    Not optimal classifier, PCA used as pre-processing method to reduce dimensionality/noise.
    """
    print("\n=== PCA BINARY CLASSIFICATION ===")

    # Compute PCA (1D - first principal component)
    P, mu = compute_pca(DTR, m=1)
    print("PCA matrix computed")

    # Project training data
    DTR_pca = apply_pca(P, mu, DTR)

    # Fix orientation (same logic as LDA)
    mean_class1 = DTR_pca[0, LTR == class1].mean()
    mean_class2 = DTR_pca[0, LTR == class2].mean()
    if mean_class1 > mean_class2:
        P = -P
        DTR_pca = -DTR_pca
        print("PCA orientation flipped to put class {} on the right".format(class2))
    else:
        print("PCA orientation is correct (class {} on the right)".format(class2))

    # Project validation data
    DVAL_pca = apply_pca(P, mu, DVAL)

    # Compute threshold
    threshold = binary_classification_threshold(DTR_pca, LTR, class1, class2)

    # Classify validation samples
    PVAL = classify_binary(DVAL_pca, threshold, class1, class2)

    # Evaluate
    evaluate_errors(PVAL, LVAL)

    return PVAL