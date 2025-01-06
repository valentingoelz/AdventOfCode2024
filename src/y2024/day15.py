from enum import Enum


class Direction(Enum):
    UP = '^'
    RIGHT = '>'
    DOWN = 'v'
    LEFT = '<'

class Object:
    def __init__(self, x, y, c):
        self.x = x
        self.y = y
        self.c = c

    def move(self, direction: Direction):
        if direction == Direction.UP:
            self.y -= 1
        elif direction == Direction.RIGHT:
            self.x += 1
        elif direction == Direction.DOWN:
            self.y += 1
        elif direction == Direction.LEFT:
            self.x -= 1


class Grid:
    def __init__(self, grid: str):
        objects = []
        for y, row in enumerate(grid):
            for x, c in enumerate(row):
                if c == '@':
                    self.robot = Object(x, y, c)
                    object = Object(x, y, '.')
                    objects.append(object)
                    continue
                object = Object(x, y, c)
                objects.append(object)
        self.objects = objects
    
    def move(self, direction: Direction):
        # find all objects in front of the robot in this direction
        objects_in_front = []
        for obj in self.objects:
            x_bot = self.robot.x
            y_bot = self.robot.y
            match direction:
                case Direction.UP:
                    if obj.y < y_bot and obj.x == x_bot:
                        objects_in_front.append(obj)
                case Direction.RIGHT:
                    if obj.x > x_bot and obj.y == y_bot:
                        objects_in_front.append(obj)
                case Direction.DOWN:
                    if obj.y > y_bot and obj.x == x_bot:
                        objects_in_front.append(obj)
                case Direction.LEFT:
                    if obj.x < x_bot and obj.y == y_bot:
                        objects_in_front.append(obj)

        # sort objects by distance to robot
        objects_in_front.sort(key=lambda obj: abs(obj.x - self.robot.x) + abs(obj.y - self.robot.y))
        # find index with first '.' or '#'
        for i, obj in enumerate(objects_in_front):
            if obj.c == '.':
                idx = i
                break
            elif obj.c == '#':
                return
        
        self.robot.move(direction)
        for obj in objects_in_front[:idx]:
            obj.move(direction)
        # move '.' to robot position
        objects_in_front[idx].x = self.robot.x
        objects_in_front[idx].y = self.robot.y


def main():
    with open("input/day15.txt") as f:
        input = f.read().strip().split("\n\n")
    
    # split grid by lines
    grid = input[0].split("\n")
    directions = input[1].replace("\n", "")
    directions = [Direction(d) for d in directions]
    print(directions)

    print("Solution Part 1: ", solve_part1(grid, directions))

def solve_part1(grid, directions):
    grid = Grid(grid)
    for direction in directions:
        grid.move(direction)
    # find all 'O' objects in the grid
    objects = [obj for obj in grid.objects if obj.c == 'O']
    total_sum = 0
    for obj in objects:
        total_sum += obj.x + 100 * obj.y
    return total_sum


if __name__ == "__main__":
    main()