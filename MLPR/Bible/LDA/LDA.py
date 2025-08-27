import numpy as np
import scipy

from mean_covariance import vcol, vrow
from PCA.PCA import apply_pca_reduction  # cartella.nome_file_py
from linear_projection_classifier import binary_classification_threshold, classify_binary, evaluate_errors


# Compute the parameters
def compute_Sb_Sw(D, L):
    Sb = 0
    Sw = 0
    muGlobal = vcol(D.mean(1))
    for i in np.unique(L):
        DCls = D[:, L == i]
        mu = vcol(DCls.mean(1))
        Sb += (mu - muGlobal) @ (mu - muGlobal).T * DCls.shape[1] # between class covariance
        Sw += (DCls - mu) @ (DCls - mu).T # within class covariance
    return Sb / D.shape[1], Sw / D.shape[1]


def compute_lda(D, L, m):
    # Compute between-class scatter matrix (Sb) and within-class scatter matrix (Sw)
    # Sb, Sw always produce a MxM matrix
    Sb, Sw = compute_Sb_Sw(D, L)
    # Solve the generalized eigenvalue problem: Sb * u = λ * Sw * u
    s, U = scipy.linalg.eigh(Sb, Sw)
    # Sort eigenvectors in descending order of eigenvalues and select the first m
    return U[:, ::-1][:, 0:m]


# def compute_lda_JointDiag(D, L, m):
#     # Other method if you want to use the Single value decomposition for rectangle matrix
#     Sb, Sw = compute_Sb_Sw(D, L)
#
#     U, s, _ = np.linalg.svd(Sw)
#     P = np.dot(U * vrow(1.0 / (s ** 0.5)), U.T)
#
#     Sb2 = np.dot(P, np.dot(Sb, P.T))
#     U2, s2, _ = np.linalg.svd(Sb2)
#
#     P2 = U2[:, 0:m]
#     return np.dot(P2.T, P).T


def apply_lda(U, D):
    """
    LDA looks for directions that separate classes, not that center the data
    - The Sb (between-class scatter) matrix already uses the differences (μᵢ - μ_global)
    - The Sw (within-class scatter) matrix uses the differences (x - μᵢ) for each class

    here D do not need to be centered
    """
    return U.T @ D


def fix_lda_orientation(U, DTR_projected, LTR, class1=1, class2=2):
    """
    Fix LDA orientation so that class2 mean > class1 mean in projected space
    """
    mean_class1 = DTR_projected[0, LTR == class1].mean()
    mean_class2 = DTR_projected[0, LTR == class2].mean()

    if mean_class1 > mean_class2:
        # Flip the sign
        U = -U
        DTR_projected = -DTR_projected
        print("LDA orientation flipped to put class {} on the right".format(class2))
    else:
        print("LDA orientation is correct (class {} on the right)".format(class2))

    return U, DTR_projected


def apply_lda_reduction(DTR, LTR, DVAL, m, class1=1, class2=2):
    """
    Perform LDA dimensionality reduction with orientation fix.
    """
    # Compute LDA projection matrix
    U = compute_lda(DTR, LTR, m=m)

    # Apply to training data
    DTR_lda = apply_lda(U, DTR)

    # Fix orientation
    U, DTR_lda = fix_lda_orientation(U, DTR_lda, LTR, class1, class2)

    # Apply to validation data
    DVAL_lda = apply_lda(U, DVAL)

    print(f"LDA: reduced to {m}D (after PCA)")

    return DTR_lda, DVAL_lda




def lda_binary_pipeline(DTR, LTR, DVAL, LVAL, class1=1, class2=2):
    """
    Complete LDA binary classification pipeline
    """
    print("=== LDA BINARY CLASSIFICATION ===")

    # Compute LDA (1D for binary problem)
    m=1
    DTR_lda, DVAL_lda = apply_lda_reduction(DTR, LTR, DVAL, m, class1, class2)

    # Compute threshold
    threshold = binary_classification_threshold(DTR_lda, LTR, class1, class2)

    # Classify validation samples
    PVAL = classify_binary(DVAL_lda, threshold, class1, class2)

    # Evaluate
    evaluate_errors(PVAL, LVAL)

    return PVAL


def pca_lda_pipeline(DTR, LTR, DVAL, LVAL, pca_dim=2, class1=1, class2=2):
    """
    PCA + LDA pipeline: first reduce with PCA, then apply LDA
    """
    print(f"\n=== PCA({pca_dim}D) + LDA PIPELINE ===")

    # Step 1: Apply PCA
    DTR_pca, DVAL_pca = apply_pca_reduction(DTR, DVAL, pca_dim)

    # Step 2: Apply LDA on PCA-reduced data (1D for binary problem)
    m=1
    DTR_pca_lda, DVAL_pca_lda = apply_lda_reduction(DTR_pca, LTR, DVAL_pca, m, class1, class2)

    # Compute threshold
    threshold = binary_classification_threshold(DTR_pca_lda, LTR, class1, class2)

    # Classify
    PVAL = classify_binary(DVAL_pca_lda, threshold, class1, class2)

    # Evaluate
    evaluate_errors(PVAL, LVAL)

    return PVAL