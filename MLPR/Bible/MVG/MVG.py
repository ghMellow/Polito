import numpy as np

from mean_covariance import vcol, vrow, compute_mu_C                #for vcol, vrow, compute_mu_C functions
from scipy.special import logsumexp                                 #for scipy.special.logsumexp
# Se stessa cartella bisogna fare import relativo o spezzare
# import logpdf_loglikelihood_GAU
# from logpdf_loglikelihood_GAU import logpdf_GAU_ND
from .logpdf_loglikelihood_GAU import logpdf_GAU_ND                 #for computing the log-likelihood of the Gaussian distribution


# 1° compute params from the model assumption: MVG, Naive, Tied

def computeParams_ML(D, labels):
    #Compute the ML (Maximum Likelihood) parameters of the MVG distribution given the data and the labels
    """
    Parameters:
    - D: the data matrix of shape (numFeatures, numSamples)
    - labels: the labels of the data, so a list of length numSamples

    Returned Values:
    - params: the model parameters, so  list of tuples (mu, C) where mu is the mean vector fo class c and C is the covariance matrix of class c
    """

    params = []
    numClasses = np.unique(labels).shape[0] #number of classes
    for label in range(numClasses):
        #compute MLE estimates of mean and covariance matrix for each class i
        params.append(compute_mu_C(D[:, labels == label])) 

    return params #params is a list of tuples (mu, C) where mu is the mean vector fo class c and C is the covariance matrix of class c


def computeParams_ML_NaiveBayesAssumption(D, labels):
    #Compute the ML (Maximum Likelihood) parameters of the MVG distribution given the data and the labels, and use the Naive Bayes assumption (so Covariance Matrices are diagonal)
    """
    Parameters:
    - D: the data matrix of shape (numFeatures, numSamples)
    - labels: the labels of the data, so a list of length numSamples

    Returned Values:
    - params: the model parameters, so  list of tuples (mu, C) where mu is the mean vector fo class c and C is diagonal the covariance matrix of class c
    """

    params = []
    numClasses = np.unique(labels).shape[0] #number of classes
    for label in range(numClasses):
        #compute MLE estimates of mean and covariance matrix for each class i
        mu, C = compute_mu_C(D[:, labels == label])
        params.append((mu, np.diag(np.diag(C)))) #append the mean vector and the diagonal covariance matrix to the list of parameters

    return params #params is a list of tuples (mu, C) where mu is the mean vector fo class c and C is the diagonal covariance matrix of class c


def computeSw(D, L):
    '''
    Params:
    - D: Dataset features matrix, not ceCntered
    - L: Labels of the samples

    Returned Values:
    - Sw: Within-class scatter matrix
    '''

    # find the unique labels for each class
    uniqueLabels = np.unique(L)

    # nc in the formula is computed as the number of samples of class c
    # separate data into classes
    DC = [D[:, L == label] for label in uniqueLabels]  # DC[0] -> samples of class 0, DC[1] -> samples of class 1 etc...

    # compute nc for each class
    # each element in DC has a shape which is (4, DC_i.shape[1]) (assuming samples are not equally distributed among all the classes which is true in 99% of cases...)
    # So for nc I just have to take DC_i.shape[1] for each i in DC
    nc = [DC_i.shape[1] for DC_i in DC]

    # Compute the mean as done before with PCA
    mu = D.mean(axis=1)
    mu = mu.reshape((mu.shape[0], 1))

    # Now compute the mean for each class
    muC = [DC[label].mean(axis=1) for label, labelName in enumerate(uniqueLabels)]
    muC = [mc.reshape((mc.shape[0], 1)) for mc in muC]

    Sw = 0  # within  matrix initialization

    # iterate over all the classes to execute the summations to calculate the Sw matrix
    for label, labelName in enumerate(uniqueLabels):
        # add up to the Sw (within) matrix
        # for diff1 subtract the the class mean from the samples of each class, i.e center center the samples for each class
        diff1 = DC[label] - muC[label]  # x_{c, i} - muC done by rows

        # SHORTCUT: compute the Sw matrix as a weighted sum of the covariance matrices of each class
        # so for each class:
        # Compute the Covariance Matrix C using DC = D - mu
        C_i = (diff1 @ diff1.T) / float(diff1.shape[1])  # Covariance matrix for class i

        # weighted sum of all the C_i
        Sw += nc[label] * C_i

    # at the end of the summations, just multiply by 1/N (N is the number of samples)
    Sw = Sw / D.shape[1]

    # return both matrices
    return Sw


def computeParams_ML_TiedCov(D, labels, useLDAForTiedCov=False):
    # Compute the ML (Maximum Likelihood) parameters of the MVG Tied Covariance model
    """
    Parameters:
    - D: the data matrix of shape (numFeatures, numSamples)
    - labels: the labels of the data, so a list of length numSamples
    - useLDAForTiedCov: if True, compute the covariance matrix using the LDA method, else compute the covariance matrix summing all the Covariance of each class * Nc and dividing by N

    Returned Values:
    params:
    - CTied: the tied covariance matrix of shape (numFeatures, numFeatures) which is the same for all classes
    - mu: the mean vectors of shape (numFeatures, numClasses) where each column is the mean vector of the class c
    """

    params = []
    classes = np.unique(labels)  # number of classes

    if (useLDAForTiedCov):
        # compute the covariance matrix using the LDA method
        Sw = computeSw(D, labels)
        for label in classes:
            # compute MLE meanst of each class i
            mu, _ = compute_mu_C(D[:, labels == label])
            params.append((mu, Sw))

        return params

    else:
        CTied = 0  # initialize the tied covariance matrix
        muVect = {}  # initialize the mean vectors dict
        for label in classes:
            # compute MLE estimates of mean and covariance matrix for each class i
            D_c = D[:, labels == label]
            Nc = D_c.shape[1]  # Nc is the number of samples of class c
            mu, C = compute_mu_C(D_c)
            muVect[label] = mu  # store the mean vector of class c
            CTied += Nc * C

        # at the end do: CTied / N
        CTied = CTied / D.shape[1]  # N = D.shape[1] is the number of samples

        # put everything in the params list
        for label in classes:
            params.append((muVect[label], CTied))

        return params



# 2° compute the score matrix S with the (log?)-pdf of each class

def scoreMatrix_Pdf_GAU(D, params, useLog=True):
    # Compute the (log?)-Pdf of the data given the parameters of a Gaussian distribution and populate the score matrix S with the (log?)-pdf of each class
    """
    Parameters:
    - D: the data matrix of shape (numFeatures, numSamples)
    - params: the model parameters, so  list of tuples (mu, C) where mu is the mean vector fo class c and C is the covariance matrix of class c
    - useLog: if True, compute the log-pdf, else compute the pdf

    Returned Values:
    - S: the score matrix of shape (numClasses, numSamples) where each row is the score of the class given the sample

    """

    # The score matrix is filled with the pdfs of the training data given the MLE parameters of the MVG distribution
    # S[i, j] is the pdf of the j-th sample given the i-th class

    numClasses = len(params)  # number of classes, since for each class we have a tuple (mu, C)
    S = np.zeros((numClasses, D.shape[1]))
    for label in range(numClasses):
        if useLog:
            # if useLog is True, then compute the log-pdf
            S[label, :] = logpdf_GAU_ND(D, params[label][0], params[label][1])
        else:
            # if useLog is False, then compute the pdf
            S[label, :] = np.exp(logpdf_GAU_ND(D, params[label][0], params[label][1]))

    return S



# 3° use the Bayes approach to make inference

def computeSJoint(S, Priors, useLog=True):
    # Compute the joint densities by multiplying the score matrix S with the Priors
    """
    Parameters:
    - S: the score matrix of shape (numClasses, numSamples) where each row is the score of the class given the sample
    - Priors: the priors of the classes, so a list of length numClasses
    - useLog: if True, compute the log-joint densities, else compute the joint densities

    Returned Values:
    - SJoint: the (log?)joint densities of shape (numClasses, numSamples) where each row is the joint density of the class given the sample
    """

    if (useLog):
        # S needs to be already in log scale, so we just need to add the log of the priors
        return S + vcol(np.log(Priors))  # multiply each row of S (where 1 row corresponds to a class) with the prior of the class
    else:
        return S * vcol(Priors)


def computePosteriors(SJoint, useLog=True):
    """
    Compute the posteriors by normalizing the joint densities
    The posteriors are the joint densities divided by the sum of the joint densities which are the marginals

    Parameters:
    - SJoint: the joint densities of shape (numClasses, numSamples) where each row is the joint density of the class

    Returned Values:
    - SPost: the posteriors of shape (numClasses, numSamples) where each row is the posterior of the class given the sample
    """
    if useLog:
        # 1. Compute marginals usign the logsumexp trick to minimize numerical problems
        # logsumexp is a function that computes the log of the sum of exponentials of input elements
        # It is more numerically stable than computing the sum of exponentials directly
        # It computes log(exp(a) + exp(b)) in a numerically stable way

        # sum over the rows (axis=0) to get the marginal of each sample
        # aka denominator of the Bayes function
        SMarginal = logsumexp(SJoint, axis=0)

        # SMarginal has now shape = (numSamples, ) -> it's a row vector
        # I need to make it of shape (1, numSamples)
        SPost = SJoint - vrow(SMarginal)  # element wise division in log scale, so I just need to subtract the marginals from the joint densities


    else:

        # 1. Compute marginals
        SMarginal = vrow(SJoint.sum(0))  # sum over the rows (axis=0) to get the marginal of each sample

        # 2. Compute posteriors by dividing the joint densities by the marginals
        SPost = SJoint / SMarginal  # element wise division

    return SPost


def classify(SPost):
    # Classify the samples by taking the class with the highest posterior probability
    """
    Parameters:
    - SPost: the posteriors of shape (numClasses, numSamples) where each row is the posterior of the class given the sample

    Returned Values:
    - labels: the predicted labels of the samples, so a list of length numSamples
    """
    return np.argmax(SPost, axis=0)


def gaussian_bayes_classifier(LTR, DVAL, ML_params, useLog=True):
    """
    Implements a Gaussian Bayesian classifier for multivariate classification.

    Applies Bayes' theorem using multivariate Gaussian densities to compute
    posterior probabilities and classify validation samples.

    Args:
        LTR (numpy.ndarray): Training set labels (1D array)
        DVAL (numpy.ndarray): Validation features matrix (d x N)
        ML_params (list): ML estimated parameters (means and covariances per class)
        useLog (bool): If True, uses logarithmic domain for numerical stability

    Returns:
        numpy.ndarray: predicted_labels - 1D array with predicted labels for each sample
    """
    # Compute score matrix (the likelihood) using the density function
    S_LogLikelihoods = scoreMatrix_Pdf_GAU(DVAL, ML_params, useLog)

    # Choose the prior distribution
    # Uniform prior distribution, equal for each class
    classes = np.unique(LTR)
    Priors = np.ones(len(classes)) / len(classes)
    # Other prior distributions can be implemented here

    # Compute the joint matrix: likelihood * prior
    S_Joint = computeSJoint(S_LogLikelihoods, Priors, useLog)

    # Compute the posterior probabilities
    S_Post = computePosteriors(S_Joint, useLog)

    # Classify samples by selecting the class with highest posterior probability for each sample
    # set axis=0 to select the class with the highest posterior probability for each sample
    predicted_labels = classify(S_Post)

    return predicted_labels


def compute_error_MVG(predicted_labels, LVAL, print_err=False):
    # Compute error rate
    error_count = np.count_nonzero(predicted_labels != LVAL)
    error_rate = np.mean(predicted_labels != LVAL)
    if print_err:
        print(f"Error Rate: {error_rate:.2%}")
        print(f"Number of wrong predictions: {error_count}")

    return error_rate


# -----------------------------------------------------------------------
# Choose the pipeline based on the model assumption
# It changes only the ML_params computation

def pipeline(DTR, LTR, DVAL, LVAL):
    # 1°
    # compute MVG parameters using Maximum Likelihood Estimation (MLE)
    # or rather finds the most probable parameters for a probability distribution, given a set of observed data, by maximizing the likelihood function A.K.A mean and covariance.
    ML_params = computeParams_ML(DTR, LTR)

    # 2° predict label for each sample
    Predicted_labels = gaussian_bayes_classifier(LTR, DVAL, ML_params)

    # 3°
    error = compute_error_MVG(Predicted_labels, LVAL)

    return Predicted_labels, error


def pipeline_Naive(DTR, LTR, DVAL, LVAL):
    # 1°
    # compute MVG parameters using Maximum Likelihood Estimation (MLE)
    ML_params = computeParams_ML_NaiveBayesAssumption(DTR, LTR)

    # 2° predict label for each sample
    Predicted_labels = gaussian_bayes_classifier(LTR, DVAL, ML_params)

    # 3°
    error = compute_error_MVG(Predicted_labels, LVAL)

    return Predicted_labels, error


def pipeline_TiedCov(DTR, LTR, DVAL, LVAL, useLDAForTiedCov=False):
    # 1°
    # compute MVG parameters using Maximum Likelihood Estimation (MLE)
    ML_params = computeParams_ML_TiedCov(DTR, LTR, useLDAForTiedCov)

    # 2° predict label for each sample
    Predicted_labels = gaussian_bayes_classifier(LTR, DVAL, ML_params)

    # 3°
    error = compute_error_MVG(Predicted_labels, LVAL)

    return Predicted_labels, error