# Knowledge:
## Node:
- **Must** have between and including 2 and 4 child node
    - Only the leafs don't have child
- **Must** have one parent node
    - Only the root doesn't have a parent
- Each node **must** have a state
    - The state of the grid at this point
    - **Should** contains location of tile `0`
- **Must** implement a method to generate new node from the previous one

### Node representation:
```Rust
struct Node {
    state: State,
    grid: Grid,
    parent: Option<Node>,
}
```

## State:
- The **cost** (`g score`) of this node which is the number of moves needed to reach this state
- The **heuristic** (`h score`) which will depends on the heuristic function we use
- The sum of the cost and the heuristic of this node (`f score`)

### State representation:
```Rust
struct State {
    h: u16,
    g: u32,
    f: u32,
}
```

## Grid:
- Represent the state of the whole puzzle for a given node
- Will be stored in each node
- **Must** implement a method to `move` a tile inside the grid (swap the position of a tile with the tile `0` if they are directly next to each other)
- **Should** keep the size of the puzzle or the greatest index we can have
- **Should** implement a method to generate a "snail" for the given puzzle size

### Grid representation:
```Rust
struct Grid {
    map: Vec<u16>,
    z_pos: u16, // Absolute position
}
```

## Closed List:
- Contains all nodes for which we have already process all their child nodes
- Will be used to construct the final path across the nodes which will give us the moves we have to do to solve the puzzle

## Open List:
- **Must** be sorted by `f score` value (lowest first) or for two nodes with the same `f score` value sorted by their heuristic values

## Algo:
- **Must** implement a main loop over the open list to unstack it
- For each node from the open list generate a new node for every possible move and push it into the open list

### Moves:
- `Up` -> tile `0` goes up
- `Down` -> tile `0` goes down
- `Right` -> tile `0` goes right
- `Left` -> tile `0` goes left
- The tile cannot go outside of the table (obviously)

## How we use axis
|      | x0 | x1 | x2 |
|------|----|----|----|
|**y0**|  1 |  2 |  3 |
|**y1**|  4 |  5 |  6 |
|**y2**|  7 |  8 |  0 |

# Ideas:
- **closedList** should probably be a BTreeSet (for read access speed)
- **openList** should probably be a BinaryHeap (for auto-sort and quick read access)
- we probably need a tree to represent all the nodes and their parent
- 

# Note:
## Bonuses:
- Limite on the research depending on `g score` or/and `h score`