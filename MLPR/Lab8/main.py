import numpy as np
from sklearn.metrics import confusion_matrix
from tabulate import tabulate
import matplotlib.pyplot as plt


def print_confusion_matrix(true_labels, predicted_labels):
    # Compute confusion matrix (true as rows, predicted as columns)
    cm = confusion_matrix(true_labels, predicted_labels)

    # Transpose to match the format: rows = predicted, columns = true|classi
    cm = cm.T

    # Prepare header
    class_labels = list(range(cm.shape[0]))
    headers = ["Prediction \\ Class"] + class_labels

    # Prepare table rows
    table = [[i] + list(row) for i, row in enumerate(cm)]

    # Print using tabulate
    print(tabulate(table, headers=headers, tablefmt="grid"))

    return cm

def print_table(headers, *columns):
    """
    Print a table using tabulate with arbitrary number of columns.

    Parameters:
    - headers: list of column headers
    - *columns: any number of lists, one for each column
    """
    rows = list(zip(*columns))  # Transpose columns into rows
    print(tabulate(rows, headers=headers, tablefmt="grid"))

def optimal_bayes_decisions(llr, pi1, Cfn, Cfp):
    """
    Computes optimal Bayes decisions for binary classification based on log-likelihood ratios.

    Parameters:
    - llr: array of log-likelihood ratios
    - pi1: prior probability of class 1 (HT)
    - Cfn: cost of false negative (predict 0 when true is 1)
    - Cfp: cost of false positive (predict 1 when true is 0)

    Returns:
    - predictions: binary array (0 or 1)
    """
    threshold = -np.log((pi1 * Cfn) / ((1 - pi1) * Cfp))
    return (llr > threshold).astype(int)

def compute_bayes_risk_DCF(confusion_matrix, pi1, Cfn, Cfp, epsilon=0.001):
    """
    Computes the Bayes risk from the confusion matrix and given costs and prior.
    Also called Detection Cost Function, DCF

    Parameters:
    - confusion_matrix: 2x2 numpy array (rows = true labels, cols = predicted labels)
    - pi1: prior of class 1 (HT)
    - Cfn: cost of false negative
    - Cfp: cost of false positive
    - epsilon: pseudocount, un piccolo valore positivo aggiunto a numeratore e denominatore nel calcolo delle probabilità. Evita divisioni per zero (ad esempio, quando una classe non ha esempi positivi/negativi) e stabilizza le stime (prevenendo che Pfn o Pfp vadano a 0 o 1 troppo rapidamente, rendendo la DCF più robusta). In pratica:
        - ε = 0.001 è una scelta conservativa: altera poco i conti, ma previene problemi numerici.
        - ε = 1 “ammorbidisce” le probabilità, riducendo la varianza.

    Returns:
    - bayes_risk: float

    Note: denominator is the sum of the column, extract and sum single value or use .sum() function
        Pfn= FN / (FN + TP)
        Pfp= FP / (FP + TN)
    """
    fn = confusion_matrix[0, 1]  # true 1, predicted 0
    fp = confusion_matrix[1, 0]  # true 0, predicted 1
    n1 = confusion_matrix[1].sum()
    n0 = confusion_matrix[0].sum()

    # if and epsilon avoid division by 0
    Pfn = (fn + epsilon) / (n1 + 2 * epsilon) if n1 > 0 else 0 # false negative rate
    Pfp = (fp + epsilon) / (n0 + 2 * epsilon) if n0 > 0 else 0 # false positive rate

    # DCF computation and min is for dummy DCF used to normalize DCF
    DCF = pi1 * Cfn * Pfn + (1 - pi1) * Cfp * Pfp
    DCF_normalized = DCF / min(pi1 * Cfn, (1 - pi1) * Cfp)
    return DCF, DCF_normalized

def compute_min_DCF(llr, labels, pi1, Cfn, Cfp, epsilon=0.001):
    # L'intervallo t ∈ {−∞, s₁, ..., s_M, +∞} viene definito automaticamente usando tutti i valori dei tuoi LLR,
    # ti assicura di testare tutte le soglie rilevanti.
    """
    In pratica è un algoritmo che prende un intervallo discreto da meno infinito e più inifinito e in maniera
    brute force calcola per ogni soglia il DCF e restitusce il minimo per una tripletta pi, cfn, cfp
    """
    thresholds = np.concatenate(([float('-inf')], np.sort(llr), [float('+inf')]))
    min_dcf = np.inf
    for t in thresholds:
        predictions = (llr > t).astype(int)
        cm = confusion_matrix(labels, predictions)
        _, dcf_norm = compute_bayes_risk_DCF(cm, pi1, Cfn, Cfp, epsilon)
        if dcf_norm < min_dcf:
            min_dcf = dcf_norm
    return min_dcf

def compute_roc_curve(llr, labels):
    thresholds = np.concatenate(([-np.inf], np.sort(llr), [np.inf]))
    tpr_list = []
    fpr_list = []

    for t in thresholds:
        preds = (llr > t).astype(int)
        cm = confusion_matrix(labels, preds).T  # predicted vs true

        fn = cm[0, 1]
        fp = cm[1, 0]
        tp = cm[1, 1]
        tn = cm[0, 0]

        n_pos = tp + fn
        n_neg = tn + fp

        Pfn = fn / n_pos if n_pos > 0 else 0
        Pfp = fp / n_neg if n_neg > 0 else 0
        Ptp = 1 - Pfn

        tpr_list.append(Ptp)
        fpr_list.append(Pfp)

    return np.array(fpr_list), np.array(tpr_list)

def plot_roc_curve(fpr, tpr):
    plt.figure(figsize=(8, 6))
    plt.plot(fpr, tpr, label="ROC Curve", color="blue")
    plt.plot([0, 1], [0, 1], linestyle='--', color='gray', label="Random Classifier")
    plt.xlabel("False Positive Rate (FPR)")
    plt.ylabel("True Positive Rate (TPR)")
    plt.title("ROC Curve")
    plt.grid(True)
    plt.legend()
    plt.tight_layout()
    plt.show()


def calculate_dcf_values(llr_data, labels_data, prior_log_odds, epsilon=0.001, Cfn=1, Cfp=1):
    """
    Calcola i valori DCF e min_DCF per i dataset forniti.

    Args:
        llr_data: Array dei log-likelihood ratio per il primo modello
        labels_data: Array delle etichette per il primo modello
        prior_log_odds: Array dei valori di log-odds per i prior effettivi
        epsilon: Valore di epsilon per il secondo modello (default: 0.001)

    Returns:
        dcf, min_dcf
    """
    dcf = []
    min_dcf = []
    for p in prior_log_odds:
        # Make sure predictions are binary (0 or 1)
        pi_eff = 1 / (1 + np.exp(-p))
        predictions = optimal_bayes_decisions(llr_data, pi_eff, Cfn, Cfp)

        # Ensure labels_data is also binary
        binary_labels = labels_data.astype(int)

        # Now both inputs to confusion_matrix are binary
        cm = confusion_matrix(binary_labels, predictions)

        # In binary task (π1,Cfn,Cfp) is indeed equivalent to an application (˜π,1,1)
        _, dcf_norm = compute_bayes_risk_DCF(cm, pi_eff, Cfn, Cfp, epsilon)
        dcf.append(dcf_norm)

        min_dcf_val = compute_min_DCF(llr_data, binary_labels, pi_eff, Cfn, Cfp, epsilon)
        min_dcf.append(min_dcf_val)

    return dcf, min_dcf


if __name__ == '__main__':
    # Initial multiclass task
    print(" >< " * 30)
    print()
    print("Multiclass - uniform priors and costs - confusion matrix")
    # Load data: log-likelihood
    commedia_ll = np.load('./Data/commedia_ll.npy')  # shape: (num_classes, num_samples)
    commedia_labels = np.load('./Data/commedia_labels.npy')  # shape: (num_samples,)
    # Predict by choosing class with highest log-likelihood (uniform priors and costs)
    predicted_labels = np.argmax(commedia_ll, axis=0)
    # Compute confusion matrix
    print_confusion_matrix(commedia_labels, predicted_labels)


    # Binary task
    print()
    print(" >< " * 30)
    print()
    print("Binary task")
    # Load data: log-likelihood ratio
    commedia_llr_binary = np.load('./Data/commedia_llr_infpar.npy')
    commedia_labels_binary = np.load('./Data/commedia_labels_infpar.npy')
    # Prediction with different parameters
    configs = [
        # (π1, Cfn, Cfp) -> (prior probability, cost of false negative, cost of false positive)
        (0.5, 1, 1),
        (0.8, 1, 1),
        (0.5, 10, 1),
        (0.8, 1, 10)
    ]
    DCF_risks = []
    DCF_normalized_risks = []
    for i, (pi1, Cfn, Cfp) in enumerate(configs, 1):
        # Predict using optimal Bayes decision rule with non-uniform priors and costs
        # on other words predict by minimizing expected Bayes risk given (π1, Cfn, Cfp)
        predicted_labels = optimal_bayes_decisions(commedia_llr_binary, pi1, Cfn, Cfp)
        # Compute confusion matrix
        print(f"\nConfig {i}) π1={pi1}, Cfn={Cfn}, Cfp={Cfp}")
        cm_result = print_confusion_matrix(commedia_labels_binary, predicted_labels)  # Rows = true labels, columns = predicted labels
                                                                                             # Entry [i,j] = number of samples from class i predicted as class j
                                                                                             # False negatives = [1,0], false positives = [0,1]
        # Binary task: evaluation
        DCF, DCF_normalized = compute_bayes_risk_DCF(cm_result, pi1, Cfn, Cfp)
        DCF_risks.append(DCF)
        DCF_normalized_risks.append(DCF_normalized)

    # Binary task: evaluation print
    print("\nEvaluation DCF")
    headers = ["(π1, Cfn, Cfp)", "DCF"]
    print_table(headers, configs, DCF_risks)
    print("Note: The Bayes risk allows us comparing diﬀerent systems, however it does not tell us what is the benefit of using\n"
          "      our recognizer with respect to optimal decisions based on prior information only. We can compute a normalized\n"
          "      detection cost, by dividing the Bayes risk by the risk of an optimal system that does not use the test data at all (dummy DCF). "
          "\n\nEvaluation Normalized DCF")
    headers = ["(π1, Cfn, Cfp)", "DCF normalized"]
    print_table(headers, configs, DCF_normalized_risks)
    print("Note: We can observe that only in two cases the DCF is lower than 1, in the remaining cases our system is actually harmful.")

    # Binary task: minimum detection costs
    # Problema a volte il training set non rappresenta le caratteristiche del dataset e
    # la soglia teorica calcolata con optimal_bayes_decisions non è la migliore
    # [dal punto di vista di Discriminazione (quanto bene separa le classi) e Calibrazione (quanto bene i punteggi riflettono vere probabilità o LLR)]

    # Compute minimum DCFs
    print("\nMinimum DCF evaluation")
    min_DCFs = []
    for (pi1, Cfn, Cfp) in configs:
        min_dcf = compute_min_DCF(commedia_llr_binary, commedia_labels_binary, pi1, Cfn, Cfp)
        min_DCFs.append(min_dcf)

    headers = ["(π1, Cfn, Cfp)", "min DCF"]
    print_table(headers, configs, min_DCFs)
    print("Note: With the except of the first application, we can observe a significant loss due to poor calibration. "
          "\nThis loss is even more significant for the two applications which had a normalized DCF larger than 1. "
          "\nIn these two scenarios, our classifier is able to provide discriminant scores, but we were not able to employ"
          "\nthe scores to make better decisions than those that we would make from the prior alone.")

    # ROC curve
    fpr, tpr = compute_roc_curve(commedia_llr_binary, commedia_labels_binary)
    plot_roc_curve(fpr, tpr)

    # Bayes Error Plot
    # Il ciclo su effPriorLogOdds rappresenta diversi scenari applicativi, da fortemente sbilanciati verso una classe a fortemente sbilanciati verso l’altra.
    # Per ogni valore:
    #    Si calcola la soglia teorica da Bayes.
    #    Si calcola il DCF normalizzato usando quella soglia.
    #    Si calcola anche il minDCF, cioè la migliore DCF ottenibile se si fosse scelta la soglia ottimale.
    # Si tracciano entrambe per visualizzare:
    #    L'effettiva performance (curva rossa)
    #    Il limite inferiore (curva blu), utile per valutare la calibrazione del sistema.

    # Load binary classification data (LLR scores and labels)
    commedia_llr_binary_eps1 = np.load('./Data/commedia_llr_infpar_eps1.npy')
    commedia_labels_binary_eps1 = np.load('./Data/commedia_llr_infpar_eps1.npy')
    # Create 21 evenly spaced log-odds values between -3 and 3
    effPriorLogOdds = np.linspace(-3, 3, 30)
    epsilon = 1.0
    dcf, min_dcf = calculate_dcf_values(
        commedia_llr_binary,
        commedia_labels_binary,
        effPriorLogOdds
    )
    dcf_eps1, min_dcf_eps1 = calculate_dcf_values(
        commedia_llr_binary_eps1,
        commedia_labels_binary_eps1,
        effPriorLogOdds,
        epsilon
    )
    print()
    headers = ["(π1, Cfn, Cfp)", "DCF", "ε= 0.001\nmin DCF", "DCF", "ε= 1\nmin DCF"]
    print_table(headers, effPriorLogOdds, dcf, min_dcf, dcf_eps1, min_dcf_eps1)
    # Plotting
    plt.figure(figsize=(8, 5))
    plt.plot(effPriorLogOdds, dcf, label='DCF (ε=0.001)', color='r')
    plt.plot(effPriorLogOdds, min_dcf, label='min DCF (ε=0.001)', color='b')
    plt.plot(effPriorLogOdds, dcf_eps1, label='DCF (ε=1)', color='y')
    plt.plot(effPriorLogOdds, min_dcf_eps1, label='min DCF (ε=1)', color='g')
    plt.xlabel('Effective Prior Log-Odds')
    plt.ylabel('Normalized DCF')
    plt.title('Bayes Error Plot')
    plt.ylim([0, 1.1])
    plt.xlim([-3, 3])
    plt.grid(True)
    plt.legend()
    plt.tight_layout()
    plt.show()