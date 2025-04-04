import matplotlib.pyplot as plt
import numpy as np

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
    print(f"Mean:\n{mu}\nCovariance matrix:\n{C}\n")

    return mu, C


# Compute log-density for a single sample x (column vector). The result is a 1-D array with 1 element
def logpdf_GAU_ND_singleSample(x, mu, C):
    M = x.size # shape[0] o shape[1] equivalenti, valore singolo
    xc = x - mu
    C_inv = np.linalg.inv(C)
    return (- M / 2 * np.log(2 * np.pi)
            - 1 / 2 * np.linalg.slogdet(C)[1] # 0: sign of the determinant # 1: absolute value of the determinant
            - 1 / 2 * (xc.T @ C_inv @ xc).ravel()) # Trasforma un array multidimensionale in un array unidimensionale (vettore): 2x3 -> vet dim 6

# Compute log-densities for N samples, arranged as a MxN matrix X (N stacked column vectors). The result is a 1-D array with N elements, corresponding to the N log-densities
def logpdf_GAU_ND_slow(X, mu, C):
    ll = [logpdf_GAU_ND_singleSample(X[:, i:i+1], mu, C) for i in range(X.shape[1])]
    return np.array(ll).ravel()

# Compute log-densities for N samples, arranged as a MxN matrix X (N stacked column vectors). The result is a 1-D array with N elements, corresponding to the N log-densities
def logpdf_GAU_ND_fast(x, mu, C):
    M = x.shape[0] # x è una matrice con dimensione (M, N), dove M è il numero di variabili e N il numero di campioni.
    xc = x - mu
    C_inv = np.linalg.inv(C)

    return (- M / 2 * np.log(2 * np.pi)
            - 1 / 2 * np.linalg.slogdet(C)[1]  # 0: sign of the determinant # 1: absolute value of the determinant
            - 1 / 2 * (xc * (C_inv @ xc)).sum(0))


def loglikelihood(XND, m_ML, C_ML):
    ll = logpdf_GAU_ND_slow(XND, m_ML, C_ML)
    return np.sum(ll)


if __name__ == '__main__':
    # - Multivariate Gaussian density -
    plt.figure()
    XPlot = np.linspace(-8, 12, 1000)
    m = np.ones((1,1)) * 1.0
    C = np.ones((1,1)) * 2.0
    
    logpdf_GAU_ND = logpdf_GAU_ND_fast(vrow(XPlot), m, C)
    
    plt.plot(XPlot.ravel(), np.exp(logpdf_GAU_ND))
    # plt.show()

    # You can also check whether your density is correct by comparing your values with those contained in Solution/llGAU.npy
    # The result should be zero or very close to zero (it may not be exactly zero due to numerical errors,
    # however it should be a very small number, e.g. ≈10−17)
    pdfSol = np.load('Solution/llGAU.npy')
    pdfGau = logpdf_GAU_ND_fast(vrow(XPlot), m, C)
    print(np.abs(pdfSol - pdfGau).max())

    # You can also check the density for the multi-dimensional case using the samples contained in Solution/XND.npy:
    # Again, the result should be zero or close to zero.
    XND = np.load('Solution/XND.npy')
    mu = np.load('Solution/muND.npy')
    C = np.load('Solution/CND.npy')
    pdfSol = np.load('Solution/llND.npy')
    pdfGau = logpdf_GAU_ND_fast(XND, mu, C)
    print(np.abs(pdfSol - pdfGau).max())


    # - Maximum Likelihood Estimate -
    m_ML, C_ML = compute_mean_covariance(XND)
    ll = loglikelihood(XND, m_ML, C_ML)
    print(f"ll: {ll} should match -270.70478023795044")

    # A second dataset corresponding to the file ’Solution/X1D.npy’ contains one-dimensional samples
    # We can visualize how well the estimated density fits the samples plotting both the histogram of the
    # samples and the density (again, m_ML and C_ML are the ML estimates):
    plt.figure()
    X1D = np.load('Solution/X1D.npy')
    m_ML, C_ML = compute_mean_covariance(X1D)
    #m_ML = np.array([1.9539157])
    #C_ML = np.array([6.09542485])
    plt.hist(X1D.ravel(), bins=50, density=True)
    XPlot = np.linspace(-8, 12, 1000)
    plt.plot(XPlot.ravel(), np.exp(logpdf_GAU_ND_fast(vrow(XPlot), m_ML, C_ML)))
    plt.show()
    # In this case, the log-likelihood for the ML estimates is
    ll = loglikelihood(X1D, m_ML, C_ML)
    print(f"ll: {ll} should match -23227.077654602715")
