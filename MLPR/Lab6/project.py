import matplotlib.pyplot as plt
import numpy
import numpy as np
import seaborn as sns
from pip._internal.utils.misc import tabulate

from main import split_db_2to1, BinaryTasks_loglikelihood_ratios_with_MVG, BinaryTasks_loglikelihood_ratios_with_TG


def load_fingerprint():
    L = []  # label: true finger, false finger
    D = []  # dataset: 6 dimension features
    with open("trainData.txt", "r") as f:
        for line in f.readlines():
            row = [column.strip() for column in line.split(",")]

            label = int(row[-1])
            features = np.array([float(feature) for feature in row[:-1]])

            L.append(label)
            D.append(features.reshape(features.shape[0], 1))

        #  righe da attaccare una di fianco all'altra -> horizontal stack
        D = np.hstack(D)
        # also L need to become a numpy object
        L = np.array(L, dtype=np.int32)

        return D, L


def vcol(row):
    return row.reshape((row.shape[0], 1))

def vrow(row):
    return row.reshape((1, row.shape[0]))


def compute_mean_covariance(D):
    # Compute mean vector
    mu = vcol(D.mean(1))
    # Center the data by subtracting the mean
    Dc = D - mu
    # Compute covariance matrix
    C = (Dc @ Dc.T) / float(Dc.shape[1])

    return mu, C


# Compute log-density for a single sample x (column vector). The result is a 1-D array with 1 element
# Il codice `logpdf_GAU_ND_singleSample` implementa la log-densità della Gaussiana multivariata, generalizzando il caso univariato a qualunque dimensione N.
# La sigla PDF sta per Probability Density Function
#       - é una funzione fondamentale in probabilità continua: non fornisce una probabilità diretta, ma indica quanto è "denso" o probabile trovare un valore vicino a un certo punto.
#       - Nel caso della **Gaussiana**, la PDF restituisce un valore più alto vicino alla media $\mu$ e decresce in modo simmetrico.
#       - Nel contesto multivariato, la PDF tiene conto anche delle **correlazioni tra le variabili** attraverso la matrice di covarianza $\Sigma$, rendendola più adatta a modellare dati reali rispetto al caso univariato o naive.
# La sigla ND chiarisce che si tratta di una funzione adatta a dati multidimensionali, in cui vengono considerate anche le correlazioni tra le feature attraverso la matrice di covarianza C.
# L'uso del logaritmo migliora la stabilità numerica, evitando problemi di underflow nei calcoli probabilistici.
# Compute log-densities for N samples, arranged as a MxN matrix X (N stacked column vectors). The result is a 1-D array with N elements, corresponding to the N log-densities

def logpdf_GAU_ND_fast(x, mu, C):
    M = x.shape[0] # x è una matrice con dimensione (M, N), dove M è il numero di variabili e N il numero di campioni.
    xc = x - mu
    C_inv = np.linalg.inv(C)

    return (- M / 2 * np.log(2 * np.pi)
            - 1 / 2 * np.linalg.slogdet(C)[1]  # 0: sign of the determinant # 1: absolute value of the determinant
            - 1 / 2 * (xc * (C_inv @ xc)).sum(0))

def BinaryTasks_loglikelihood_ratios_with_NG(DTR, LTR, DTE, LTE):
    # Nive Gaussian
    unique_classes = numpy.unique(LTR)
    # Verifichiamo che ci siano esattamente 2 classi
    assert len(unique_classes) == 2, "Expected binary classification task"

    class_0, class_1 = unique_classes

    # Calcolo parametri per classe 0
    D_class_0 = DTR[:, LTR == class_0]
    mu_0, C_0 = compute_mean_covariance(D_class_0)
    C_0_diag = np.diag(np.diag(C_0))

    # Calcolo parametri per classe 1
    D_class_1 = DTR[:, LTR == class_1]
    mu_1, C_1 = compute_mean_covariance(D_class_1)
    C_1_diag = np.diag(np.diag(C_1))

    # Calcolo dei log-likelihood (non dei likelihood)
    ll_0 = logpdf_GAU_ND_fast(DTE, mu_0, C_0_diag)
    ll_1 = logpdf_GAU_ND_fast(DTE, mu_1, C_1_diag)

    # Calcolo del rapporto di log-likelihood
    LLR = ll_1 - ll_0

    # Predizioni
    PVAL = numpy.zeros(DTE.shape[1], dtype=numpy.int32)
    TH = 0
    PVAL[LLR >= TH] = class_1
    PVAL[LLR < TH] = class_0

    # Calcolo dell'errore
    error_rate = (PVAL != LTE).sum() / float(LTE.size) * 100
    print(f"MVG - Error rate: {error_rate:.1f}%")

    return error_rate

def pearson_correlation_matrix(D):
    # Normalize each row (feature) to zero mean and unit std
    D_centered = D - D.mean(axis=1, keepdims=True)
    D_std = D.std(axis=1, keepdims=True)
    D_normalized = D_centered / D_std

    # Compute correlation matrix as the covariance of normalized features
    return (D_normalized @ D_normalized.T) / D.shape[1]

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


if __name__ == '__main__':
    # load dataset of true/false fingerprint
    D, L = load_fingerprint()

    (DTR, LTR), (DTE, LTE) = split_db_2to1(D, L)


    BinaryTasks_loglikelihood_ratios_with_MVG(DTR, LTR, DTE, LTE) # Assunzione distribuzione gaussiana
    BinaryTasks_loglikelihood_ratios_with_TG(DTR, LTR, DTE, LTE)  # Covarianza uguale, assume che all the sample clusters are spread in the same way around their mean
                                                                  #   se guardi grafici dei primi lab vedi che ultime due feature del dataset non sono raggruppate vicino alla media
    BinaryTasks_loglikelihood_ratios_with_NG(DTR, LTR, DTE, LTE)  # Assume che i sample siano indipendenti perciò prendo solo la diagonale della matrice covarianza

    labels = np.unique(LTR)
    for label in labels:
        D_label = DTR[:, LTR == label]
        C_pearson = pearson_correlation_matrix(D_label) # grafico mostra che le feature sono poco correlate singola label

        plt.figure(figsize=(6, 5))
        plt.title(f"Pearson Correlation Matrix - Label {label}")
        sns.heatmap(C_pearson, annot=True, fmt=".2f", cmap='Blues', vmin=-1, vmax=1, xticklabels=range(1, C_pearson.shape[0] + 1), yticklabels=range(1, C_pearson.shape[0] + 1))
        plt.tight_layout()
        plt.show()


    # Gaussian models goodness evaluation.
    # qui vengono tolte a coppie varie feature e si vede come reagiscono le classificazioni binarie. Si ottiene che le
    # coppie più discriminanti (se tolte peggiorano risultati classificazione) sono:
    #   1) f1, f2
    #   2) f3, f4
    #   3) f5, f6 (tied gaus. è meglio qui rispetto f3,f4. Dovuto al fatto che f5,f6 hanno varianza alta, ossia distanti dalla media mentre tied assume una varianza bassa).


    # Classification with PCA as pre-processing.
    # ora usiamo pca, lda e pca + lda per mitigare l'effetto negativo delle feature negative.
    MVG = []
    TG = []
    NG = []
    for m in range(1, DTR.shape[0]+1): # [1, 6) ossia 1 a 5 P prende da 0 salto caso 0-0
        # Compute pca
        mu, C = compute_mean_covariance(DTR)
        U, s, Vh = numpy.linalg.svd(C)
        P = U[:, 0:m]
        # Projecting the data usando stesso P calcolato da DTR
        DTR_pca = project_dataset(P, DTR)
        DTE_pca = project_dataset(P, DTE)

        MVG.append(BinaryTasks_loglikelihood_ratios_with_MVG(DTR_pca, LTR, DTE_pca, LTE))
        TG.append(BinaryTasks_loglikelihood_ratios_with_TG(DTR_pca, LTR, DTE_pca, LTE))
        NG.append(BinaryTasks_loglikelihood_ratios_with_NG(DTR_pca, LTR, DTE_pca, LTE))

    print(f"\n{'PCA dim':>8} | {'MVG':>10} | {'Tied G':>10} | {'Naive G':>10}")
    print("-" * 47)
    for m in range(DTR.shape[0]):
        print(f"{m+1:>8} | {MVG[m]:>10.3f} | {TG[m]:>10.3f} | {NG[m]:>10.3f}")


    # As we can see, PCA leads to worse or equal results for both multivariate gaussian and naive gaussian,
    # while for tied gaussian, by using 𝑚 ∈ [2,4], we get slightly better results (9.25% error rate vs. 9.30%
    # without PCA). The overall best result (MVG model with 𝑚 = 6) has a 7.00% error rate, which is the
    # same as the one without PCA; in fact, in this case, since 𝑚 = 6, there is no actual dimensionality
    # reduction but the dataset gets transformed according to the principal directions, which does not aﬀect
    # the model performance.
    # On the other hand we can say that, given that the worsening in the error rates is not too high (even
    # for low values of 𝑚), applying PCA for pre-processing can be a good compromise if we need to reduce
    # the dimensionality of the dataset.