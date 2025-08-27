import numpy as np


def binary_classification_threshold(DTR_projected, LTR, class1=1, class2=2):
    """
    Compute threshold as mean of projected class means
    """
    mean_class1 = DTR_projected[0, LTR == class1].mean()
    mean_class2 = DTR_projected[0, LTR == class2].mean()
    threshold = (mean_class1 + mean_class2) / 2.0

    print(f"Class {class1} projected mean: {mean_class1:.4f}")
    print(f"Class {class2} projected mean: {mean_class2:.4f}")
    print(f"Threshold: {threshold:.4f}")

    return threshold


def classify_binary(D_projected, threshold, class1=1, class2=2):
    """
    Binary classification using threshold

    PVAL = vector of predicted labels
    """
    PVAL = np.zeros(D_projected.shape[1], dtype=np.int32)
    PVAL[D_projected[0] >= threshold] = class2
    PVAL[D_projected[0] < threshold] = class1
    return PVAL


def evaluate_errors(PVAL, LVAL):
    """
    Count classification errors

    LVAL = vector of real labels
    """
    errors = np.sum(PVAL != LVAL)
    error_rate = errors / len(LVAL)
    print(f"Errors: {errors} out of {len(LVAL)} samples")
    print(f"Error rate: {error_rate:.4f} ({error_rate*100:.2f}%)")
    return errors, error_rate