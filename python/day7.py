file1 = open('input/y2022q7.txt', 'r').read()
lines = file1.split('\n')

import collections

cache = collections.defaultdict(int)
stack = []
result = 0

for i in lines:
    if i[0] == "$":
        cmd = i.split(" ")
        if cmd[1] == "cd" and cmd[2] == "..":
            stack.pop()
        elif cmd[1] == "cd" and cmd[2] != "/":
            stack.append(cmd[2])
    elif i[0].isnumeric():
        for j in range(len(stack) + 1):
            cache["/".join(stack[0:j])] += int(i.split(" ")[0])


for k, l in cache.items():
    if l < 100000:
        result += l

curr_remaining = 70000000 - cache[""]
more_needed = 30000000 - curr_remaining

smallest = 70000000
for k, l in cache.items():
    if l >= more_needed and k != "":
        smallest = min(l, smallest)

print(result)
print(smallest)
