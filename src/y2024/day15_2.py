from enum import Enum
import copy

class Direction(Enum):
    UP = '^'
    RIGHT = '>'
    DOWN = 'v'
    LEFT = '<'

class Object(Enum):
    WALL = '#'
    OPEN = '.'
    BoxLeft = '['
    BoxRight = ']'
    Robot = '@'


def main():
    with open("../../input/day15.example") as f:
        input = f.read().strip().split("\n\n")
    
    # split grid by lines
    grid = input[0].split("\n")
    new_grid = []
    for row in grid:
        row = row.replace("#", "##")
        row = row.replace("O", "[]")
        row = row.replace(".", "..")
        row = row.replace("@", "@.")
        new_grid.append(row)


    directions = input[1].replace("\n", "")
    directions = [Direction(d) for d in directions]

    print("Solution Part 2: ", solve_part2(new_grid, directions))

def find_robot_position(grid):
    for y, row in enumerate(grid):
        for x, c in enumerate(row):
            if c == '@':
                return x, y

def handle_left(grid):
    x, y = find_robot_position(grid)
    left_objects = grid[y][0:x]
    for i in range(len(left_objects)-1, -1, -1):
        if left_objects[i] == '#':
            return grid
        if left_objects[i] == '.':
            row = left_objects[:i] + left_objects[i+1:] + '@' + "." + grid[y][x+1:]
            grid[y] = row
            return grid
    assert False, "This should not happen"

def handle_right(grid):
    x, y = find_robot_position(grid)
    right_objects = grid[y][x+1:]
    for i in range(len(right_objects)):
        if right_objects[i] == '#':
            return grid
        if right_objects[i] == '.':
            row = grid[y][0:x] + "." + '@' +  right_objects[:i] + right_objects[i+1:]
            grid[y] = row
            return grid
    assert False, "This should not happen"

def handle_up(grid):
    robot_x, robot_y = find_robot_position(grid)
    points_to_check = [(robot_x, robot_y-1)]
    new_grid = copy.deepcopy(grid)
    while points_to_check:
        x, y = points_to_check.pop()
        if grid[y][x] == '#':
            return grid
        if grid[y][x] == '.':
            new_grid[y, x] = grid[y+1, x]
            new_grid[y+1, x] = '.'
        if grid[y][x] == '[':
            points_to_check.append((x, y-1), (x+1, y-1))
            new_grid[y, x] = grid[y+1, x]
        if grid[y][x] == ']':
            points_to_check.append((x, y-1), (x-1, y-1))
            new_grid[y, x] = grid[y+1, x]


##[]....[]....[]..##
##..[][]..[]..[][]##
##........[]...[].##
##........[]....@.##
####################


##[]....[]........##
##........[]...[@.##
##........[]....@.##
####################






def solve_part2(grid, directions):
    print("\n".join(grid))
    interesting_idx = list(range(len(directions)))
    interesting_directions = [Direction.RIGHT]
    for i, direction in enumerate(directions[:35]):
        old_grid = copy.deepcopy(grid)
        match direction:
            case Direction.UP:
                pass
            case Direction.RIGHT:
                grid = handle_right(grid)
            case Direction.DOWN:
                pass
            case Direction.LEFT:
                grid = handle_left(grid)
        if i in interesting_idx:
            if direction in interesting_directions:
                print(direction)
                print("\n".join([old_row + "    " + new_row for old_row, new_row in zip(old_grid, grid)]))
                print()

if __name__ == "__main__":
    main()