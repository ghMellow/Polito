import numpy
import scipy


def vcol(row):
    return row.reshape((row.shape[0], 1))

def vrow(row):
    return row.reshape((1, row.shape[0]))

def load_dataset(fname):
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
                attrs = vcol(attrs)

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

def split_db_2to1(D, L, seed=0):
    nTrain = int(D.shape[1]*2.0/3.0)
    numpy.random.seed(seed)
    idx = numpy.random.permutation(D.shape[1])
    idxTrain = idx[0:nTrain]
    idxTest = idx[nTrain:]
    DTR = D[:, idxTrain]
    DTE = D[:, idxTest]
    LTR = L[idxTrain]
    LTE = L[idxTest]
    return (DTR, LTR), (DTE, LTE)

def compute_mean_covariance(D):
    # Compute mean vector
    mu = vcol(D.mean(1))
    # Center the data by subtracting the mean
    Dc = D - mu
    # Compute covariance matrix
    C = (Dc @ Dc.T) / float(Dc.shape[1])

    return mu, C

# Compute log-densities for N samples, arranged as a MxN matrix X (N stacked column vectors). The result is a 1-D array with N elements, corresponding to the N log-densities
# !Logaritmo della funzione di densità gaussiana!
def logpdf_GAU_ND_fast(x, mu, C):
    M = x.shape[0] # x è una matrice con dimensione (M, N), dove M è il numero di variabili e N il numero di campioni.
    xc = x - mu
    C_inv = numpy.linalg.inv(C)

    return (- M / 2 * numpy.log(2 * numpy.pi)
            - 1 / 2 * numpy.linalg.slogdet(C)[1]  # 0: sign of the determinant # 1: absolute value of the determinant
            - 1 / 2 * (xc * (C_inv @ xc)).sum(0))


def Multivariate_Gaussian_Classifier(DTR, LTR, DTE, LTE):
    # - Multivariate Gaussian Classifier -
    # (MVG) As we have seen, the classifier assumes that samples of each class c ∈{0,1,2}can be modeled as samples of a
    # multivariate Gaussian distribution with class-dependent mean and covariance matrices.
    unique_classes = numpy.unique(LTR)    # Numero di classi uniche da Labels
    num_features = DTR.shape[0]           # Numero di caratteristiche nel Dataset

    S = []  # Matrix to store the likelihoods (class conditional probability) of each class.
            # Each row of the score matrix corresponds to a class, and contains the conditional log-likelihoods for all the samples for that class.
    prior_probability = 1 / 3

    for i, class_label in enumerate(unique_classes):
        D_class = DTR[:, LTR == class_label]

        # (1) Compute the ML estimates for the classifier parameters (µ0,Σ0),(µ1,Σ1),(µ2,Σ2).
        mu, C = compute_mean_covariance(D_class)  # NOTE: TRAINING PHASE.
        print(f"\nClass {class_label} \nmean: \n{mu} \ncovariance: \n{C}")

        # (2) Given the estimated model, we now turn our attention towards inference for a test sample x. As we
        # have seen, the final goal is to compute class posterior probabilities P(c|x). We split the process in three
        # stages. The first step consists in computing, for each test sample, the likelihoods.
        ll = numpy.exp(logpdf_GAU_ND_fast(DTE, mu, C))  # NOTE: INFERENCE PHASE ON TEST DATA.

        # (3) We can now compute class posterior probabilities combining the score matrix with prior information. In
        # the following we assume that the three classes have the same prior probability P(c) = 1 /3. We can thus
        # compute the joint distribution for samples and classes.
        S.append(ll * prior_probability)

    SJoint = numpy.vstack(S) # ovvero fX,C (xt,c) = fX|C (xt|c)PC (c)

    # Check SJoint with solution
    pdfSol = numpy.load('Solution/SJoint_MVG.npy')
    is_equal = numpy.allclose(SJoint, pdfSol)
    print(f"I risultati sono uguali: {is_equal}")
    if not is_equal:
        max_diff = numpy.max(numpy.abs(SJoint - pdfSol))
        print(f"Differenza massima: {max_diff}")

    # (4) Finally, we can compute class posterior probabilities as P(C= c|X= xt) = fX,C (xt,c) / Sommatoria di c′ fX,C (xt,c′)
    # Ovvero calcolo delle probabilità posteriori equivale a P(C=c|X=xt) = SJoint / SMarginal
    SMarginal = vrow(SJoint.sum(0)) # ovvero fX (xt) = sommatoria c fX,C (xt,c).
    SPosterior = SJoint / SMarginal

    # Classification
    # Predizione delle etichette come classe con la massima probabilità posteriore
    predicted_labels = numpy.argmax(SPosterior, axis=0)

    # Calcolo dell'accuratezza
    correct_predictions = (predicted_labels == LTE).sum()
    total_samples = LTE.size
    accuracy = correct_predictions / total_samples

    # Calcolo del tasso di errore
    error_rate = 1 - accuracy

    # Stampa dei risultati in percentuale
    print(f"Accuratezza: {accuracy * 100:.1f}%")
    print(f"Tasso di errore: {error_rate * 100:.1f}%")


def Naive_Bayes_Gaussian_Classifier(DTR, LTR, DTE, LTE):
    # We now consider the Naive Bayes version of the classifier. As we have seen, the Naive Bayes version of
    # the MVG is simply a Gaussian classifier where the covariance matrices are diagonal.

    # Same code but Covariance matrix we preserve only the diagonal value
    unique_classes = numpy.unique(LTR)  # Use training labels
    num_features = DTR.shape[0]  # Use training features

    # Store class parameters (trained on training data)
    class_parameters = []

    # Train the model (estimate parameters from training data)
    for class_label in unique_classes:
        DTR_class = DTR[:, LTR == class_label]
        mu, C = compute_mean_covariance(DTR_class)

        C = C * numpy.identity(num_features)

        class_parameters.append((mu, C))
        print(f"\nClass {class_label} \nmean: \n{mu} \ncovariance: \n{C}")

    # Compute likelihoods on test data
    S = []
    prior_probability = 1 / 3

    for i, (mu, C) in enumerate(class_parameters):
        # Compute log-densities for all test samples using the parameters of class i
        ll = numpy.exp(logpdf_GAU_ND_fast(DTE, mu, C))
        S.append(ll * prior_probability)

    # Joint distribution
    SJoint = numpy.vstack(S)

    # Optional: Compare with solution if available
    try:
        pdfSol = numpy.load('Solution/SJoint_MVG.npy')
        is_equal = numpy.allclose(SJoint, pdfSol)
        print(f"I risultati sono uguali: {is_equal}")
        if not is_equal:
            max_diff = numpy.max(numpy.abs(SJoint - pdfSol))
            print(f"Differenza massima: {max_diff}")
    except FileNotFoundError:
        print("Solution file not found, skipping comparison")

    # Compute marginal and posterior probabilities
    SMarginal = vrow(SJoint.sum(0))
    SPosterior = SJoint / SMarginal

    # Classification
    predicted_labels = numpy.argmax(SPosterior, axis=0)

    # Evaluation
    correct_predictions = (predicted_labels == LTE).sum()
    total_samples = LTE.size
    accuracy = correct_predictions / total_samples
    error_rate = 1 - accuracy

    print(f"Accuratezza: {accuracy * 100:.1f}%")
    print(f"Tasso di errore: {error_rate * 100:.1f}%")

    # Add confusion matrix for better insight
    conf_matrix = numpy.zeros((len(unique_classes), len(unique_classes)), dtype=int)
    for i in range(len(LTE)):
        conf_matrix[LTE[i], predicted_labels[i]] += 1

    print("\nMatrice di confusione:")
    print(conf_matrix)


def compute_Sb_Sw(D, L):
    Sb = 0
    Sw = 0
    muGlobal = vcol(D.mean(1))
    for i in numpy.unique(L):
        DCls = D[:, L == i]
        mu = vcol(DCls.mean(1))
        Sb += (mu - muGlobal) @ (mu - muGlobal).T * DCls.shape[1]
        Sw += (DCls - mu) @ (DCls - mu).T
    return Sb / D.shape[1], Sw / D.shape[1]


def logpdf_GAU_ND(x, mu, C):
    P = numpy.linalg.inv(C)
    return -0.5*x.shape[0]*numpy.log(numpy.pi*2) - 0.5*numpy.linalg.slogdet(C)[1] - 0.5 * ((x-mu) * (P @ (x-mu))).sum(0)

# Compute per-class log-densities. We assume classes are labeled from 0 to C-1. The parameters of each class are in hParams (for class i, hParams[i] -> (mean, cov))
def compute_log_likelihood_Gau(D, hParams):

    S = numpy.zeros((len(hParams), D.shape[1]))
    for lab in range(S.shape[0]):
        S[lab, :] = logpdf_GAU_ND(D, hParams[lab][0], hParams[lab][1])
    return S

# compute log-postorior matrix from log-likelihood matrix and prior array
def compute_logPosterior(S_logLikelihood, v_prior):
    SJoint = S_logLikelihood + vcol(numpy.log(v_prior))
    SMarginal = vrow(scipy.special.logsumexp(SJoint, axis=0))
    SPost = SJoint - SMarginal
    return SPost

# Compute a dictionary of ML parameters for each class - Tied Gaussian model
# We exploit the fact that the within-class covairance matrix is a weighted mean of the covraince matrices of the different classes
def Gau_Tied_ML_estimates(D, L):
    labelSet = set(L)
    hParams = {}
    hMeans = {}
    CGlobal = 0
    for lab in labelSet:
        DX = D[:, L==lab]
        mu, C_class = compute_mean_covariance(DX)
        CGlobal += C_class * DX.shape[1]
        hMeans[lab] = mu
    CGlobal = CGlobal / D.shape[1]
    for lab in labelSet:
        hParams[lab] = (hMeans[lab], CGlobal)
    return hParams

def BinaryTasks_loglikelihood_ratios_and_MVG(DTR, LTR, DTE, LTE):
    None

if __name__ == '__main__':
    fname = "iris.csv"
    D, L = load_dataset(fname)

    # DTR and LTR are training data and labels, DTE and LTE are evaluation (or more precisely validation) data and labels
    (DTR, LTR), (DTE, LTE) = split_db_2to1(D, L)

    # first model
    # Multivariate_Gaussian_Classifier(DTR, LTR, DTE, LTE)

    #
    # Naive_Bayes_Gaussian_Classifier(DTR, LTR, DTE, LTE)

    hParams_Tied = Gau_Tied_ML_estimates(DTR, LTR)
    for lab in [0, 1, 2]:
        print('Tied Gaussian - Class', lab)
        print(hParams_Tied[lab][0])
        print(hParams_Tied[lab][1])
        print()

    S_logLikelihood = compute_log_likelihood_Gau(DTE, hParams_Tied)
    S_logPost = compute_logPosterior(S_logLikelihood, numpy.ones(3) / 3.)
    PVAL = S_logPost.argmax(0)
    print("Tied Gaussian - Error rate: %.1f%%" % ((PVAL != LTE).sum() / float(LTE.size) * 100))