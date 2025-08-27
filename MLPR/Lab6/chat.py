import numpy as np
import matplotlib.pyplot as plt
from sklearn.datasets import make_classification
from sklearn.model_selection import train_test_split
from sklearn.metrics import accuracy_score, classification_report, confusion_matrix
from scipy.stats import multivariate_normal
import seaborn as sns


class MVGClassifier:
    """Multivariate Gaussian Classifier (Full Covariance)"""

    def __init__(self):
        self.means_ = {}
        self.covariances_ = {}
        self.priors_ = {}
        self.classes_ = None

    def fit(self, X, y):
        """Addestra il classificatore MVG"""
        self.classes_ = np.unique(y)
        n_samples = len(y)

        for c in self.classes_:
            X_c = X[y == c]
            self.means_[c] = np.mean(X_c, axis=0)
            self.covariances_[c] = np.cov(X_c, rowvar=False)
            self.priors_[c] = len(X_c) / n_samples

    def _log_likelihood_ratio(self, X):
        """Calcola il log-likelihood ratio per la classificazione binaria"""
        classes = list(self.classes_)
        class_1, class_2 = classes[0], classes[1]

        # Log-likelihood per classe 1
        log_lik_1 = multivariate_normal.logpdf(
            X, mean=self.means_[class_1], cov=self.covariances_[class_1]
        )

        # Log-likelihood per classe 2
        log_lik_2 = multivariate_normal.logpdf(
            X, mean=self.means_[class_2], cov=self.covariances_[class_2]
        )

        # Log-likelihood ratio
        llr = log_lik_2 - log_lik_1

        # Log-odds a priori
        log_prior_odds = np.log(self.priors_[class_2] / self.priors_[class_1])

        return llr + log_prior_odds

    def predict(self, X):
        """Predice le classi usando la regola di decisione binaria"""
        llr = self._log_likelihood_ratio(X)
        classes = list(self.classes_)

        # Classificazione: classe 2 se llr >= 0, classe 1 altrimenti
        predictions = np.where(llr >= 0, classes[1], classes[0])
        return predictions

    def predict_proba(self, X):
        """Calcola le probabilità a posteriori"""
        llr = self._log_likelihood_ratio(X)
        # Converte llr in probabilità usando sigmoid
        prob_class_2 = 1 / (1 + np.exp(-llr))
        prob_class_1 = 1 - prob_class_2
        return np.column_stack([prob_class_1, prob_class_2])


class NaiveBayesClassifier:
    """Naive Bayes Classifier (Diagonal Covariance)"""

    def __init__(self):
        self.means_ = {}
        self.variances_ = {}
        self.priors_ = {}
        self.classes_ = None

    def fit(self, X, y):
        """Addestra il classificatore Naive Bayes"""
        self.classes_ = np.unique(y)
        n_samples = len(y)

        for c in self.classes_:
            X_c = X[y == c]
            self.means_[c] = np.mean(X_c, axis=0)
            self.variances_[c] = np.var(X_c, axis=0)
            self.priors_[c] = len(X_c) / n_samples

    def _log_likelihood_ratio(self, X):
        """Calcola il log-likelihood ratio assumendo indipendenza delle features"""
        classes = list(self.classes_)
        class_1, class_2 = classes[0], classes[1]

        # Log-likelihood per classe 1 (somma dei log delle pdf univariate)
        log_lik_1 = np.sum(
            -0.5 * np.log(2 * np.pi * self.variances_[class_1]) -
            0.5 * (X - self.means_[class_1]) ** 2 / self.variances_[class_1],
            axis=1
        )

        # Log-likelihood per classe 2
        log_lik_2 = np.sum(
            -0.5 * np.log(2 * np.pi * self.variances_[class_2]) -
            0.5 * (X - self.means_[class_2]) ** 2 / self.variances_[class_2],
            axis=1
        )

        # Log-likelihood ratio
        llr = log_lik_2 - log_lik_1

        # Log-odds a priori
        log_prior_odds = np.log(self.priors_[class_2] / self.priors_[class_1])

        return llr + log_prior_odds

    def predict(self, X):
        """Predice le classi usando la regola di decisione binaria"""
        llr = self._log_likelihood_ratio(X)
        classes = list(self.classes_)

        # Classificazione: classe 2 se llr >= 0, classe 1 altrimenti
        predictions = np.where(llr >= 0, classes[1], classes[0])
        return predictions

    def predict_proba(self, X):
        """Calcola le probabilità a posteriori"""
        llr = self._log_likelihood_ratio(X)
        # Converte llr in probabilità usando sigmoid
        prob_class_2 = 1 / (1 + np.exp(-llr))
        prob_class_1 = 1 - prob_class_2
        return np.column_stack([prob_class_1, prob_class_2])


def plot_decision_boundary(X, y, classifier, title, ax):
    """Visualizza la superficie di decisione"""
    h = 0.02
    x_min, x_max = X[:, 0].min() - 1, X[:, 0].max() + 1
    y_min, y_max = X[:, 1].min() - 1, X[:, 1].max() + 1
    xx, yy = np.meshgrid(np.arange(x_min, x_max, h),
                         np.arange(y_min, y_max, h))

    mesh_points = np.c_[xx.ravel(), yy.ravel()]
    Z = classifier.predict(mesh_points)
    Z = Z.reshape(xx.shape)

    ax.contourf(xx, yy, Z, alpha=0.3, cmap='RdYlBu')
    scatter = ax.scatter(X[:, 0], X[:, 1], c=y, cmap='RdYlBu', edgecolors='black')
    ax.set_title(title)
    ax.set_xlabel('Feature 1')
    ax.set_ylabel('Feature 2')
    return scatter


def compare_classifiers():
    """Confronta MVG e Naive Bayes su dataset sintetico"""

    # Genera dataset sintetico
    print("Generazione dataset sintetico...")
    X, y = make_classification(n_samples=1000, n_features=2, n_redundant=0,
                               n_informative=2, n_clusters_per_class=1,
                               class_sep=1.5, random_state=42)

    # Split train/test
    X_train, X_test, y_train, y_test = train_test_split(
        X, y, test_size=0.3, random_state=42, stratify=y
    )

    # Inizializza classificatori
    mvg_clf = MVGClassifier()
    nb_clf = NaiveBayesClassifier()

    # Addestramento
    print("Addestramento classificatori...")
    mvg_clf.fit(X_train, y_train)
    nb_clf.fit(X_train, y_train)

    # Predizioni
    mvg_pred = mvg_clf.predict(X_test)
    nb_pred = nb_clf.predict(X_test)

    # Valutazione
    print("\n" + "=" * 50)
    print("RISULTATI DELLA CLASSIFICAZIONE")
    print("=" * 50)

    print(f"\nMVG Accuracy: {accuracy_score(y_test, mvg_pred):.4f}")
    print(f"Naive Bayes Accuracy: {accuracy_score(y_test, nb_pred):.4f}")

    print("\n--- MVG Classification Report ---")
    print(classification_report(y_test, mvg_pred))

    print("\n--- Naive Bayes Classification Report ---")
    print(classification_report(y_test, nb_pred))

    # Visualizzazione
    fig, axes = plt.subplots(2, 2, figsize=(15, 12))

    # Decision boundaries
    plot_decision_boundary(X_test, y_test, mvg_clf,
                           'MVG - Decision Boundary', axes[0, 0])
    plot_decision_boundary(X_test, y_test, nb_clf,
                           'Naive Bayes - Decision Boundary', axes[0, 1])

    # Confusion matrices
    mvg_cm = confusion_matrix(y_test, mvg_pred)
    nb_cm = confusion_matrix(y_test, nb_pred)

    sns.heatmap(mvg_cm, annot=True, fmt='d', cmap='Blues',
                ax=axes[1, 0], cbar=False)
    axes[1, 0].set_title('MVG - Confusion Matrix')
    axes[1, 0].set_xlabel('Predicted')
    axes[1, 0].set_ylabel('Actual')

    sns.heatmap(nb_cm, annot=True, fmt='d', cmap='Blues',
                ax=axes[1, 1], cbar=False)
    axes[1, 1].set_title('Naive Bayes - Confusion Matrix')
    axes[1, 1].set_xlabel('Predicted')
    axes[1, 1].set_ylabel('Actual')

    plt.tight_layout()
    plt.show()

    # Analisi dei parametri appresi
    print("\n" + "=" * 50)
    print("PARAMETRI APPRESI")
    print("=" * 50)

    print("\n--- MVG Parameters ---")
    for c in mvg_clf.classes_:
        print(f"Classe {c}:")
        print(f"  Media: {mvg_clf.means_[c]}")
        print(f"  Covarianza: \n{mvg_clf.covariances_[c]}")
        print(f"  Prior: {mvg_clf.priors_[c]:.4f}")

    print("\n--- Naive Bayes Parameters ---")
    for c in nb_clf.classes_:
        print(f"Classe {c}:")
        print(f"  Media: {nb_clf.means_[c]}")
        print(f"  Varianza: {nb_clf.variances_[c]}")
        print(f"  Prior: {nb_clf.priors_[c]:.4f}")

    return mvg_clf, nb_clf, X_test, y_test


def analyze_llr_distributions(mvg_clf, nb_clf, X_test, y_test):
    """Analizza le distribuzioni dei log-likelihood ratio"""

    # Calcola LLR per entrambi i classificatori
    mvg_llr = mvg_clf._log_likelihood_ratio(X_test)
    nb_llr = nb_clf._log_likelihood_ratio(X_test)

    # Visualizzazione delle distribuzioni LLR
    fig, axes = plt.subplots(1, 2, figsize=(15, 5))

    # MVG LLR
    for class_label in np.unique(y_test):
        mask = y_test == class_label
        axes[0].hist(mvg_llr[mask], alpha=0.6, label=f'Classe {class_label}', bins=30)
    axes[0].axvline(x=0, color='red', linestyle='--', label='Soglia (LLR=0)')
    axes[0].set_xlabel('Log-Likelihood Ratio')
    axes[0].set_ylabel('Frequenza')
    axes[0].set_title('Distribuzione LLR - MVG')
    axes[0].legend()
    axes[0].grid(True, alpha=0.3)

    # Naive Bayes LLR
    for class_label in np.unique(y_test):
        mask = y_test == class_label
        axes[1].hist(nb_llr[mask], alpha=0.6, label=f'Classe {class_label}', bins=30)
    axes[1].axvline(x=0, color='red', linestyle='--', label='Soglia (LLR=0)')
    axes[1].set_xlabel('Log-Likelihood Ratio')
    axes[1].set_ylabel('Frequenza')
    axes[1].set_title('Distribuzione LLR - Naive Bayes')
    axes[1].legend()
    axes[1].grid(True, alpha=0.3)

    plt.tight_layout()
    plt.show()

    print("\n" + "=" * 50)
    print("ANALISI LOG-LIKELIHOOD RATIO")
    print("=" * 50)
    print(f"MVG LLR - Media: {np.mean(mvg_llr):.4f}, Std: {np.std(mvg_llr):.4f}")
    print(f"NB LLR - Media: {np.mean(nb_llr):.4f}, Std: {np.std(nb_llr):.4f}")


if __name__ == "__main__":
    # Esegui il confronto completo
    mvg_clf, nb_clf, X_test, y_test = compare_classifiers()

    # Analizza le distribuzioni LLR
    analyze_llr_distributions(mvg_clf, nb_clf, X_test, y_test)

    print("\n" + "=" * 50)
    print("DIFFERENZE PRINCIPALI")
    print("=" * 50)
    print("1. MVG: Usa la matrice di covarianza completa")
    print("   - Può catturare correlazioni tra features")
    print("   - Superficie di decisione quadratica")
    print()
    print("2. Naive Bayes: Assume indipendenza delle features")
    print("   - Usa solo la diagonale della covarianza")
    print("   - Superficie di decisione più semplice")
    print("   - Meno parametri da stimare")
    print()
    print("3. La regola di decisione è identica:")
    print("   - Confronto del LLR con soglia (log-odds priori)")
    print("   - Differiscono solo nel calcolo delle likelihood")