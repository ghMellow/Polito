
class Copies:
    def __init__(self, ISBN, BUY_SELL, DATE, NUM_OF_COPIES, PRICE_PER_COPY):
        self.isbn = ISBN
        self.buy_sell = BUY_SELL
        self.date = DATE
        self.num_of_copies = NUM_OF_COPIES
        self.price_per_copy = PRICE_PER_COPY


if __name__ == "__main__":
    bookstore = []
    with open("ex4_data.txt", "r") as file:
        for line in file:
            ISBN, BUY_SELL, DATE, NUM_OF_COPIES, PRICE_PER_COPY = line.split()
            bookstore.append(Copies(ISBN, BUY_SELL, DATE, int(NUM_OF_COPIES), float(PRICE_PER_COPY)))

    dict_books = {}
    dict_month_year = {}
    for book in bookstore:
        if book.isbn not in dict_books:
            if book.buy_sell == "B":
                dict_books[book.isbn] = {"isbn": book.isbn, "copies_available": book.num_of_copies, "copies_sold": 0, "gain": 0}
            else:
                dict_books[book.isbn] = {"isbn": book.isbn, "copies_available": 0, "copies_sold": book.num_of_copies, "gain": book.price_per_copy * book.num_of_copies}
        else:
            if book.buy_sell == "B":
                dict_books[book.isbn]["copies_available"] += book.num_of_copies
            else:
                dict_books[book.isbn]["copies_sold"] += book.num_of_copies
                dict_books[book.isbn]["gain"] += book.price_per_copy * book.num_of_copies

        key_month_year = f"{book.date.split("/")[1]}, {book.date.split("/")[2]}"
        if key_month_year not in dict_month_year:
            if book.buy_sell == "S":
                dict_month_year[key_month_year] = {"copies_sold": book.num_of_copies}
            else:
                dict_month_year[key_month_year] = {"copies_sold": 0}
        elif book.buy_sell == "S":
            dict_month_year[key_month_year]["copies_sold"] += book.num_of_copies

    print("Available copies and copies sold:")
    for book in dict_books:
        print(f"{book}: copies available {dict_books[book]['copies_available']} | copies sold {dict_books[book]['copies_sold']}")

    print("Sold books per month:")
    for month_year in dict_month_year:
        if dict_month_year[month_year]["copies_sold"] != 0:
            print(f"{month_year}: copies available {dict_month_year[month_year]["copies_sold"]}")

    print("Gain per book:")
    for book in dict_books:
        print(f"{book}: {dict_books[book]["gain"]} ({dict_books[book]["gain"] / dict_books[book]["copies_sold"]}, {dict_books[book]["copies_sold"]})")