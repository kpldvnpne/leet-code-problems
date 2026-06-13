class Node:
    def __init__(self, val = 0, neighbors = None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []


from typing import Optional


class Solution:
    def __init__(self):
        # Tied to the instance, resets for every test case
        self.cache = {}

    def cloneGraph(self, node: Optional['Node']) -> Optional['Node']:
        if node is None:
            print("Node is none")
            return None
        print(node.val)
        print(node.neighbors)
        if node.val in self.cache:
            return self.cache[node.val]
        new_node = Node(node.val)
        self.cache[node.val] = new_node
        new_node.neighbors = [self.cloneGraph(nbr) for nbr in node.neighbors]
        return new_node

