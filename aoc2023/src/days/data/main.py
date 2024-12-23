data = None
with open('day17_test2.txt', 'r') as fin:
    data = [[int(c) for c in line ]for line in fin.read().splitlines()]


import heapq

def shortest_path(grid, start, end):
    rows, cols = len(grid), len(grid[0])
    directions = [(0, 1), (0, -1), (1, 0), (-1, 0)]  # Right, Left, Down, Up

    # Create a 3D array to store the cost to reach each cell considering the last direction
    costs = [[[float('inf')] * 4 for _ in range(cols)] for _ in range(rows)]
    for i in range(4):
        costs[start[0]][start[1]][i] = grid[start[0]][start[1]]

    # Priority queue for the cells to visit
    queue = [(grid[start[0]][start[1]], start, -1, 0)]
    while queue:
        cost, (x, y), last_dir, count = heapq.heappop(queue)
        if [x, y] == end:
            return cost
        for i, (dx, dy) in enumerate(directions):
            nx, ny = x + dx, y + dy
            if 0 <= nx < rows and 0 <= ny < cols:
                new_cost = cost + grid[nx][ny]
                new_count = count + 1 if i == last_dir else 1
                if new_cost < costs[nx][ny][i] and new_count <= 3:
                    costs[nx][ny][i] = new_cost
                    heapq.heappush(queue, (new_cost, (nx, ny), i, new_count))
    return -1  # Return -1 if there is no path

# Test the function
grid = [
    [1, 3, 1],
    [1, 5, 1],
    [4, 2, 1]
]
grid = data
start = [0, 0]
end = [len(grid)-1, len(grid[0])-1]
print(shortest_path(grid, start, end))  # Output: 7