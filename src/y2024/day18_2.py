import networkx as nx

# import networkxnopath error
from networkx.exception import NetworkXNoPath

with open("../../input/day18.txt") as f:
    f = f.read().strip().split("\n")

size = 70
bytes = 1024
bound = size + 3

coordinates = []

print(f[2936])

for coord in f:
    coord = coord.split(",")
    x = coord[0]
    y = coord[1]
    coordinates.append((int(x)+1, int(y)+1))

grid = [["." for _ in range(bound)] for _ in range(bound)]

# make edges # as well
for i in range(bound):
    grid[0][i] = "#"
    grid[bound - 1][i] = "#"
    grid[i][0] = "#"
    grid[i][bound - 1] = "#"

grid[1][1] = "S"
grid[bound - 2][bound - 2] = "E"

fourdir = (1, -1, 1j, -1j)

for i in range(2930):
    x, y = coordinates.pop(0)
    grid[y][x] = "#"

for i, (x, y) in enumerate(coordinates):
    # print(f"{i + 2930}")
    # print(f"{x},{y}")
    grid[y][x] = "#"
    G = nx.DiGraph()
    for i, l in enumerate(grid):
        for j, c in enumerate(l):
            if c == "#":
                continue
            z = i + 1j * j
            if c == "S":
                start = (z, 1j)
            if c == "E":
                end = z
            for dz in fourdir:
                G.add_node((z, dz))

    for z, dz in G.nodes:
        if (z + dz, dz) in G.nodes:
            G.add_edge((z, dz), (z + dz, dz), weight=1)
        for rot in -1j, 1j:
            G.add_edge((z, dz), (z, dz * rot), weight=0)

    for dz in fourdir:
        G.add_edge((end, dz), "end", weight=0)

    try:
        res = nx.shortest_path_length(G, start, "end", weight="weight")
        print(res)
        print(f"{x-1},{y-1}")
    except NetworkXNoPath:
        print("No path")
        print(f"{x-1},{y-1}")
        break