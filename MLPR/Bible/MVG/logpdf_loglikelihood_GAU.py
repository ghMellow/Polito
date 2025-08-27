import numpy


# Compute log-densities for N samples, arranged as a MxN matrix X (N stacked column vectors). The result is a 1-D array with N elements, corresponding to the N log-densities
def logpdf_GAU_ND(x, mu, C):
    P = numpy.linalg.inv(C)
    return -0.5*x.shape[0]*numpy.log(numpy.pi*2) - 0.5*numpy.linalg.slogdet(C)[1] - 0.5 * ((x-mu) * (P @ (x-mu))).sum(0)

#Compute the log LIKEHOOD of the probability density function of a multivariate Gaussian distribution
def compute_ll(X, mu, C):
    return logpdf_GAU_ND(X, mu, C).sum()

