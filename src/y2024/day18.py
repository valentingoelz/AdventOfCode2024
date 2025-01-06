import networkx as nx

with open("../../input/day18.txt") as f:
    f = f.read().strip().split("\n")

size = 70
bytes = 2937
bound = size + 3

coordinates = []

for coord in f[:bytes]:
    coord = coord.split(",")
    x = coord[0]
    y = coord[1]
    coordinates.append((int(x)+1, int(y)+1))

grid = [["." for _ in range(bound)] for _ in range(bound)]
for x, y in coordinates:
    grid[y][x] = "#"

# make edges # as well
for i in range(bound):
    grid[0][i] = "#"
    grid[bound - 1][i] = "#"
    grid[i][0] = "#"
    grid[i][bound - 1] = "#"

grid[1][1] = "S"
grid[bound - 2][bound - 2] = "E"

fourdir = (1, -1, 1j, -1j)

G = nx.DiGraph()

for i, l in enumerate(grid):
    for j, x in enumerate(l):
        if x == "#":
            continue
        z = i + 1j * j
        if x == "S":
            start = (z, 1j)
        if x == "E":
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

print(nx.shortest_path_length(G, start, "end", weight="weight"))