import matplotlib.pyplot as plt
import numpy as np
import networkx as nx

with open("../../input/day12.txt") as f:
    input = f.read().strip().split("\n")
    input = [list(line) for line in input]


class Position:
    def __init__(self, row, column, character):
        self.row = row
        self.column = column
        self.character = character
        self.is_used = False

def solve_part_1(input):
    # create graph with all the positions, create node type for each character
    G = nx.Graph()
    rows = len(input)
    columns = len(input[0])
    for row in range(rows):
        for column in range(columns):
            G.add_node(Position(row, column, input[row][column]))

    # add edges between all the nodes that are of the same type and are connected
    for node in G.nodes:
        for neighbor in G.nodes:
            if node == neighbor:
                continue
            if node.character == neighbor.character:
                if abs(node.row - neighbor.row) + abs(node.column - neighbor.column) == 1:
                    G.add_edge(node, neighbor)
    
    # find all the connected components
    connected_components = list(nx.connected_components(G))
    
    # print number of connected components
    print(len(connected_components))
    total_sum = 0
    for component in connected_components:
        perimeter = 0
        for node in component:
            # get number of edges for each node
            perimeter += 4 - len(G.edges(node))
        total_sum += perimeter * len(component)
    
    print(total_sum)





solve_part_1(input)