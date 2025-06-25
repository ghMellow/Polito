class People:
    def __init__(self, name, surname, birthplace, birthdate):
        self.name=name
        self.surname=surname
        self.birthplace=birthplace
        self.birthdate=birthdate


if __name__ == '__main__':
    fname = "ex3_data.txt"
    list_people = []
    with open(fname, "r") as f:
        for line in f:
            name, surname, birthplace, birthdate = line.split()
            list_people.append(People(name, surname, birthplace, birthdate))

    dict_city = {}
    dict_month = {}
    count_birth = 0
    for people in list_people:
        # dict birth per city
        if people.birthplace not in dict_city:
            dict_city[people.birthplace] = 1
        else:
            dict_city[people.birthplace] += 1

        # dict birth per month
        if people.birthdate.split("/")[1] not in dict_month:
            dict_month[people.birthdate.split("/")[1]] = 1
        else:
            dict_month[people.birthdate.split("/")[1]] += 1
        count_birth += 1

    print("Births per city:")
    for city in dict_city:
        print(f"{city}: {dict_city[city]}")

    print("Births per month:")
    for month in dict_month:
        print(f"{month}: {dict_month[month]}")

    print(f"Average number of births: {float(count_birth)/len(dict_city):.2f}")