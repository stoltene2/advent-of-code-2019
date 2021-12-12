use std::collections::{HashMap, HashSet};

type Node = &'static str;
type Graph = HashMap<Node, HashSet<Node>>;

fn main() {
    // 1. Create hash from all inputs connecting all edges
    //     Key: Node
    //     Value: Set<Node>

    // 2. Create recursive function which returns number of visited paths
    //     - Stops if
    //       - Reaches the node called end -- returns 1 for being one complete path
    //       - `unvisited_nodes` is empty  -- returns 0 for not being a complete path

    //    Adds `current` node to incoming set of `visited_nodes` if it is a small cave
    //       - visited_nodes = visited_nodes + current
    //
    //    `unvisited_nodes` = neighbors - `visited_nodes`

    let g = build_graph(input());

    let t = explore_p1(&g, &"start", HashSet::new());

    //    println!("graph: {:?}", &g);
    println!("size: {}", &t);
}

fn explore_p1(g: &Graph, node: &Node, mut visited: HashSet<Node>) -> i32 {
    if is_small_cave(&node) {
        visited.insert(node);
    }

    let to_visit: HashSet<Node> = neighbors(&g, &node).difference(&visited).cloned().collect();

    if node == &"end" {
        return 1;
    }

    // todo: need to handle node == end
    if to_visit.is_empty() {
        return 0;
    } else {
        // TODO: calculate paths from each one in to_visit
        return to_visit.iter().fold(0, |total, next_node| {
            //node
            total + explore_p1(g, &next_node, visited.clone())
        });
    }

    // do it
}

/// lists all nodes connected to `Node` inside of `g`
fn neighbors<'a>(g: &'a Graph, n: &Node) -> HashSet<Node> {
    // Lookup edge in the map and return the HashSet

    g.get(n).unwrap().clone()
}

fn build_graph(edges: Vec<(&'static str, &'static str)>) -> Graph {
    // add starting and ending nodes as Keys
    // Merge resulting nodes to HashMap

    // use `entry` https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry
    let mut nodes: Graph = HashMap::new();

    for (from, to) in edges {
        let n = nodes.entry(from).or_insert(HashSet::new());
        n.insert(to);

        let n = nodes.entry(to).or_insert(HashSet::new());
        n.insert(from);
    }

    nodes
}

/// Determine if cave is large or small
fn is_small_cave(cave: &str) -> bool {
    cave.chars().nth(0).unwrap().is_lowercase()
}

/// Example tst from the puzzle
fn test_input() -> Vec<(&'static str, &'static str)> {
    vec![
        ("start", "A"),
        ("start", "b"),
        ("A", "c"),
        ("A", "b"),
        ("b", "d"),
        ("A", "end"),
        ("b", "end"),
    ]
}

/// Puzzle input
fn input() -> Vec<(&'static str, &'static str)> {
    vec![
        ("mj", "TZ"),
        ("start", "LY"),
        ("TX", "ez"),
        ("uw", "ez"),
        ("ez", "TZ"),
        ("TH", "vn"),
        ("sb", "uw"),
        ("uw", "LY"),
        ("LY", "mj"),
        ("sb", "TX"),
        ("TH", "end"),
        ("end", "LY"),
        ("mj", "start"),
        ("TZ", "sb"),
        ("uw", "RR"),
        ("start", "TZ"),
        ("mj", "TH"),
        ("ez", "TH"),
        ("sb", "end"),
        ("LY", "ez"),
        ("TX", "mt"),
        ("vn", "sb"),
        ("uw", "vn"),
        ("uw", "TZ"),
    ]
}
