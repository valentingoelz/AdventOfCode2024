import numpy as np
import re
import matplotlib.pyplot as plt
import matplotlib.animation as animation

with open("../../input/day14.txt") as f:
    input = f.read().strip().split("\n")

# line looks like this: 'p=0,4 v=3,-3'
class Robot:
    def __init__(self, line):
        numbers = list(map(int, re.findall(r'-?\d+', line)))
        self.position = numbers[:2]
        self.velocity = numbers[2:]
        self.x_bound = 100
        self.y_bound = 102
    
    def move(self):
        self.position[0] += self.velocity[0]
        self.position[1] += self.velocity[1]
        if self.position[0] < 0:
            self.position[0] = self.x_bound + self.position[0] + 1
        if self.position[0] > self.x_bound:
            self.position[0] = self.position[0] - self.x_bound - 1
        if self.position[1] < 0:
            self.position[1] = self.y_bound + self.position[1] + 1
        if self.position[1] > self.y_bound:
            self.position[1] = self.position[1] - self.y_bound - 1

robots = [Robot(line) for line in input]
# visualize robot positions in the grid
# create empty grid of dots

x_bound = robots[0].x_bound + 1
y_bound = robots[0].y_bound + 1

grid = np.zeros((y_bound, x_bound), dtype=int)
for robot in robots:
    grid[robot.position[1], robot.position[0]] += 1

# create animated video of the robots moving

images = []
for i in range(6577):
    grid = np.zeros((y_bound, x_bound), dtype=int)
    for robot in robots:
        robot.move()
        grid[robot.position[1], robot.position[0]] += 1

plt.imshow(grid)
plt.show()

# get each grid quadrant, leave the middle line out
x_middle = x_bound // 2
y_middle = y_bound // 2
top_left = grid[:y_middle, :x_middle]
top_right = grid[:y_middle, x_middle+1:]
bottom_left = grid[y_middle+1:, :x_middle]
bottom_right = grid[y_middle+1:, x_middle+1:]
solution = sum(top_left.flatten()) * sum(top_right.flatten()) * sum(bottom_left.flatten()) * sum(bottom_right.flatten())


