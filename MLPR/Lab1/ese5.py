import numpy

def get_neighbors1(matrix, x, y):
    # Get the number of rows and columns in the matrix
    rows, cols = matrix.shape

    # Create a list of tuples (nx, ny) representing the coordinates of neighboring cells
    return [(nx, ny)
            # Iterate over rows from x-1 to x+1 (including the row above, the current row, and the row below)
            for nx in range(x-1, x+2)
            # Iterate over columns from y-1 to y+1 (including the left column, the current column, and the right column)
            for ny in range(y-1, y+2)
            # Ensure the indices are within the matrix boundaries and exclude the central cell itself
            if (0 <= nx < rows and 0 <= ny < cols) and (nx, ny) != (x, y)]

def get_neighbors2(matrix, x, y, neighbors1):
    rows, cols = matrix.shape[0], matrix.shape[1]
    neighbors2 = []
    for nx in range(x-2, x+3):
        for ny in range(y-2, y+3):
            if (0 <= nx < rows and 0 <= ny < cols) and (nx, ny) != (x, y) and (nx, ny) not in neighbors1:
                neighbors2.append((nx, ny))
    return neighbors2

""" equivalente:
neighbors = []
for nx in range(x-1, x+2):  # Loop over rows (above, current, below)
    for ny in range(y-1, y+2):  # Loop over columns (left, current, right)
        if (0 <= nx < rows and 0 <= ny < cols) and (nx, ny) != (x, y):  # Check bounds and avoid center
            neighbors.append((nx, ny))
"""


if __name__ == "__main__":
    spot_lights = []
    with open("ex5_data.txt", "r") as f:
        nline = int(f.readline())
        matrix = numpy.zeros((nline, nline))

        for line in f:
            x, y = line.split()
            matrix[int(x)][int(y)] = 1.0
            spot_lights.append(f"{x} {y}")



    print(matrix)

    for spot_light in spot_lights:
        x, y = spot_light.split()

        print(f"{x} {y}: {get_neighbors1(matrix, int(x), int(y))} | {get_neighbors2(matrix, int(x), int(y), get_neighbors1(matrix, int(x), int(y)))}")

        for (mx, my) in get_neighbors1(matrix, int(x), int(y)):
            matrix[mx][my] += 0.5

        for (mx, my) in get_neighbors2(matrix, int(x), int(y), get_neighbors1(matrix, int(x), int(y))):
            matrix[mx][my] += 0.2

    print("Output:")
    print(matrix)