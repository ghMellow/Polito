import sys


class Athlete:
    def __init__(self, name, surname, country, scores):
        self.name = name
        self.surname = surname
        self.country = country
        self.scores = scores
        self.score = None

    def to_string(self):
        return f"{self.name} {self.surname} {self.country} {self.scores}"

    def filtered_score(self):
        min = self.scores[0]
        max = 0
        for value in self.scores:
            value = float(value)
            if value < min:
                min = value
            if value > max:
                max = value

        self.scores.remove(max)
        self.scores.remove(min)

        score = 0
        for value in self.scores:
            score += value

        self.score = score

        return f"{self.name} {self.surname}", round(score, 2)

def best_country(list_athletes):
    # dizionario: nazione, count score
    dictionary = {}
    for athlete in list_athletes:
        dictionary.setdefault(athlete.country, 0)
        dictionary[athlete.country] += athlete.score

    max_country = max(dictionary.values())
    country = next((k for k, v in dictionary.items() if v == max_country), None)
    return f"{country} - Total score: {max_country}"


if __name__ == '__main__':
    if len(sys.argv) == 2:
        # python3 ese1.py ex1_data.txt
        fname = sys.argv[1]
    else:
        fname = "ex1_data.txt"

    list_athletes = []
    with open(fname, "r") as f:
        N = int(f.readline())
        for line in f:
            name, surname, country, *values = line.split()
            scores = [float(value) for value in values]
            list_athletes.append(Athlete(name, surname, country, scores))

    [print(l.to_string()) for l in list_athletes]

    score_list = []
    [score_list.append(l.filtered_score()) for l in list_athletes]

    sorted_score_list = sorted(score_list, key=lambda x: x[1], reverse=True)

    print("Final ranking:")
    for index, sorted_athletes in enumerate(sorted_score_list, start=1):
        print(f"{index}. {sorted_athletes[0]} - score: {sorted_athletes[1]}")
        if index == 3:
            break

    print("Best Country:")
    print(best_country(list_athletes))

