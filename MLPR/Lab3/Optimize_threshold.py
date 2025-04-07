import numpy as np
from sklearn.metrics import accuracy_score, balanced_accuracy_score, f1_score, confusion_matrix


"""
Caratteristiche principali della funzione:

Calcola la soglia iniziale come media delle medie delle due classi
Esplora un intervallo di valori attorno a questa soglia iniziale
Per ogni soglia, calcola diverse metriche (accuracy, balanced accuracy, F1 score)
Seleziona la soglia che ottimizza la metrica scelta
Fornisce statistiche dettagliate sulla performance del classificatore con la soglia ottimale

Puoi personalizzare la ricerca modificando i seguenti parametri:

metric: scegli tra 'accuracy', 'balanced_accuracy' o 'f1'
range_factor: aumentalo per esplorare un intervallo più ampio
num_points: aumentalo per una ricerca più granulare

La funzione plot_threshold_metrics è inclusa per visualizzare graficamente come variano le metriche al cambiare della soglia, aiutandoti a identificare il punto ottimale.
"""

def optimize_threshold(DTR_lda, LTR, DVAL_lda, LVAL,
                       metric='accuracy',
                       range_factor=0.2,
                       num_points=50,
                       return_metrics=False):
    """
    Ottimizza la soglia di decisione esplorando valori vicini al valore iniziale
    per minimizzare l'errore di predizione.

    Parametri:
    - DTR_lda: Proiezioni LDA del training set
    - LTR: Etichette del training set
    - DVAL_lda: Proiezioni LDA del validation set
    - LVAL: Etichette del validation set
    - metric: Metrica da ottimizzare ('accuracy', 'balanced_accuracy', 'f1', 'min_dcf')
    - range_factor: Fattore che determina l'intervallo di esplorazione intorno alla soglia iniziale
    - num_points: Numero di punti da esplorare nell'intervallo
    - return_metrics: Se True, restituisce anche le metriche per ogni soglia esplorata

    Restituisce:
    - best_threshold: La soglia che ottimizza la metrica scelta
    - best_metric_value: Il valore della metrica ottimizzata
    - threshold_metrics: (opzionale) Un dizionario con liste di soglie e valori delle metriche
    """
    # Calcolo della soglia iniziale (media delle medie delle classi)
    mu_false_fingerprint = DTR_lda[0, LTR == 0].mean()
    mu_true_fingerprint = DTR_lda[0, LTR == 1].mean()
    initial_threshold = (mu_true_fingerprint + mu_false_fingerprint) / 2.0

    # Definizione dell'intervallo di esplorazione
    min_threshold = initial_threshold - range_factor * abs(mu_true_fingerprint - mu_false_fingerprint)
    max_threshold = initial_threshold + range_factor * abs(mu_true_fingerprint - mu_false_fingerprint)
    thresholds = np.linspace(min_threshold, max_threshold, num_points)

    # Inizializzazione delle liste per memorizzare i risultati
    accuracy_list = []
    balanced_accuracy_list = []
    f1_list = []

    # Calcolo delle metriche per ogni soglia
    for threshold in thresholds:
        # Predizioni con la soglia corrente
        PVAL = np.zeros(shape=LVAL.shape, dtype=np.int32)
        PVAL[DVAL_lda[0] >= threshold] = 1  # predicted as true
        PVAL[DVAL_lda[0] < threshold] = 0  # predicted as false

        # Calcolo delle metriche
        acc = accuracy_score(LVAL, PVAL)
        bal_acc = balanced_accuracy_score(LVAL, PVAL)
        f1 = f1_score(LVAL, PVAL)

        accuracy_list.append(acc)
        balanced_accuracy_list.append(bal_acc)
        f1_list.append(f1)

    # Selezione della metrica appropriata
    if metric == 'accuracy':
        metric_values = accuracy_list
    elif metric == 'balanced_accuracy':
        metric_values = balanced_accuracy_list
    elif metric == 'f1':
        metric_values = f1_list
    else:
        raise ValueError(f"Metrica '{metric}' non supportata")

    # Trovare la soglia con il valore metrico migliore
    best_idx = np.argmax(metric_values)
    best_threshold = thresholds[best_idx]
    best_metric_value = metric_values[best_idx]

    # Calcolo delle predizioni con la soglia ottimale
    PVAL_optimized = np.zeros(shape=LVAL.shape, dtype=np.int32)
    PVAL_optimized[DVAL_lda[0] >= best_threshold] = 1
    PVAL_optimized[DVAL_lda[0] < best_threshold] = 0

    # Calcolo della matrice di confusione
    conf_matrix = confusion_matrix(LVAL, PVAL_optimized)

    print(f"Soglia ottimale: {best_threshold:.4f}")
    print(f"Valore {metric}: {best_metric_value:.4f}")
    print(f"Matrice di confusione:\n{conf_matrix}")

    # Calcolo dell'accuratezza per classe
    if conf_matrix.shape == (2, 2):
        true_negative = conf_matrix[0, 0]
        false_positive = conf_matrix[0, 1]
        false_negative = conf_matrix[1, 0]
        true_positive = conf_matrix[1, 1]

        accuracy_class0 = true_negative / (true_negative + false_positive) if (
                                                                                          true_negative + false_positive) > 0 else 0
        accuracy_class1 = true_positive / (true_positive + false_negative) if (
                                                                                          true_positive + false_negative) > 0 else 0

        print(f"Accuratezza classe 0: {accuracy_class0:.4f}")
        print(f"Accuratezza classe 1: {accuracy_class1:.4f}")

    if return_metrics:
        threshold_metrics = {
            'thresholds': thresholds,
            'accuracy': accuracy_list,
            'balanced_accuracy': balanced_accuracy_list,
            'f1': f1_list
        }
        return best_threshold, best_metric_value, threshold_metrics

    return best_threshold, best_metric_value


def plot_threshold_metrics(threshold_metrics):
    """
    Funzione per visualizzare graficamente l'andamento delle metriche al variare della soglia.

    Parametri:
    - threshold_metrics: Dizionario contenente le liste di soglie e i valori delle metriche
    """
    import matplotlib.pyplot as plt

    thresholds = threshold_metrics['thresholds']
    accuracy = threshold_metrics['accuracy']
    balanced_accuracy = threshold_metrics['balanced_accuracy']
    f1 = threshold_metrics['f1']

    plt.figure(figsize=(12, 6))
    plt.plot(thresholds, accuracy, 'b-', label='Accuracy')
    plt.plot(thresholds, balanced_accuracy, 'g-', label='Balanced Accuracy')
    plt.plot(thresholds, f1, 'r-', label='F1 Score')

    plt.xlabel('Soglia')
    plt.ylabel('Valore metrica')
    plt.title('Metriche al variare della soglia')
    plt.legend()
    plt.grid(True)

    # Individuazione del punto di massimo per ogni metrica
    max_acc_idx = np.argmax(accuracy)
    max_bal_acc_idx = np.argmax(balanced_accuracy)
    max_f1_idx = np.argmax(f1)

    plt.axvline(x=thresholds[max_acc_idx], color='b', linestyle='--', alpha=0.5)
    plt.axvline(x=thresholds[max_bal_acc_idx], color='g', linestyle='--', alpha=0.5)
    plt.axvline(x=thresholds[max_f1_idx], color='r', linestyle='--', alpha=0.5)

    plt.show()