import numpy

if __name__ == "__main__":
    list_athletes = []
    list_scores = []
    fname = "ex8_data.txt"
    with open(fname, "r") as f:
        N = int(f.readline())
        for line in f:
            name, surname, country, *values = line.split()
            scores = [float(value) for value in values]
            scores.remove(min(scores))
            scores.remove(max(scores))
            
            # np_scores = numpy.array(scores)
            np_scores = numpy.array(scores).reshape(len(scores), 1)

            list_athletes.append(f"{name} {surname}")
            list_scores.append(np_scores)

    # create matrix
    # matrix_scores = numpy.vstack(list_scores)
    matrix_scores = numpy.hstack(list_scores)

    print(list_athletes)
    print(matrix_scores)

    # array_scores = numpy.sum(matrix_scores, axis=1) # sum the elements in the matrix by rows
    array_scores = numpy.sum(matrix_scores, axis=0) # sum the elements in the matrix by columns
    print(f"\nvettore puntaggi: {array_scores}")

    # first 3 competitors
    index_ordered = numpy.argsort(array_scores)[::-1] # to taking them in reverse order
    first_three = index_ordered[:3] # just the firsts three index

    print("\nfinal ranking:")
    for position, index in enumerate(first_three, start=1):
        print(f"{position}: {list_athletes[index]} - {array_scores[index]}")
