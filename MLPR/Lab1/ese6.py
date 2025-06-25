import numpy as np

def get_neighbors1(matrix, x, y):
    rows, cols = matrix.shape
    neighbors1 = []
    for nx in range(x-1, x+2):
        for ny in range(y-1, y+2):
            if (0 <= nx < rows and 0 <= ny < cols) and (nx, ny) != (x, y):
                neighbors1.append((nx, ny))
    return neighbors1


def get_neighbors2(matrix, x, y, neighbors1):
    rows, cols = matrix.shape
    neighbors2 = []
    for nx in range(x-2, x+3):
        for ny in range(y-2, y+3):
            if (0 <= nx < rows and 0 <= ny < cols) and (nx, ny) != (x, y) and (nx, ny) not in neighbors1:
                neighbors2.append((nx, ny))
    return neighbors2


if __name__ == "__main__":
    spot_lights = []
    with open("ex5_data.txt", "r") as f:
        nline = int(f.readline())
        matrix = np.zeros( (nline, nline), dtype=float )

        for line in f.readlines():
            x, y = line.split()
            spot_lights.append( (int(x), int(y)) )
            matrix[int(x), int(y)] = 1

    print(spot_lights)
    print(matrix)

    for spot_light in spot_lights:
        x, y = spot_light[0], spot_light[1]

        for (mx, my) in get_neighbors1(matrix, x, y):
            matrix[mx][my] += 0.5

        for (mx, my) in get_neighbors2(matrix, x, y, get_neighbors1(matrix, x, y)):
            matrix[mx][my] += 0.2

    print("Output:")
    print(matrix)