import numpy

def create_martix(m, n):
    list_obj = []
    for i in range(m):
        row = []
        for j in range(n):
            row.append(i*j)

        list_obj.append(numpy.array(row))

    return numpy.vstack(list_obj) # vertical stack: unisce una lista di array aggiungendo i campi come nuove righe


def normalize_column(matrix):
    row, column = matrix.shape

    normalized_matrix = numpy.zeros((row, column))
    for j in range(column):
        sum=0
        for i in range(row):
            sum += matrix[i][j]

        # sum : 1 = matrix[i][j] : x --> x = matrix[i][j] / sum
        for i in range(row):
            normalized_matrix[i][j] = matrix[i][j]/sum

    return normalized_matrix

def normalize_row(matrix):
    row, column = matrix.shape

    normalized_matrix = numpy.zeros((row, column))
    for i in range(row):
        sum=0
        for j in range(column):
            sum += matrix[i][j]

        # sum : 1 = matrix[i][j] : x --> x = matrix[i][j] / sum
        for j in range(column):
            normalized_matrix[i][j] = matrix[i][j]/sum

    return normalized_matrix

def positive_matrix(matrix):
    row, column = matrix.shape

    positive_matrix = numpy.zeros((row, column))
    for i in range(row):
        for j in range(column):
            if matrix[i][j] < 0:
                positive_matrix[i][j] = 0
            else:
                positive_matrix[i][j] = matrix[i][j]

    return positive_matrix

def product_matrix(matrix1, matrix2):
    return numpy.dot(matrix1, matrix2)



if __name__ == "__main__":
    print("a.")
    print(create_martix(3, 4))

    matrix = numpy.array([[1.0, 2.0, -6.0, 4.0],
                          [3.0, 4.0, -3.0, 7.0],
                          [1.0, 4.0, -6.0, 9.0]])
    print("b.")
    print(normalize_column(matrix))
    print("c.")
    print(normalize_row(matrix))
    print("d.")
    print(positive_matrix(matrix))
    print("e.")                                 # row:0, column:1 --trasposta--> 1, 0
    print(product_matrix(matrix, matrix.reshape(matrix.shape[1], matrix.shape[0]))) # seconda matrice trasposta