from functools import cache

words = set()
patterns = []

@cache
def isValid(pattern):
    if not pattern:
        return 1
    
    count = 0
    for i in range(1, len(pattern) + 1):
        prefix = pattern[0:i]
        suffix = pattern[i:]
        
        if prefix in words:
            count += isValid(suffix)
    
    return count
    

file_path = '../../input/day19.txt'

with open(file_path, 'r') as f:
    file = f.readlines()
    words = set(file[0].strip().split(", "))
    patterns = [pattern.strip() for pattern in file[2:]]

count = 0
for pattern in patterns:
    count += isValid(pattern)
print(count)
