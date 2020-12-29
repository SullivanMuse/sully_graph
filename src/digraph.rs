#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Node(usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Edge(usize, usize);

#[derive(Debug)]
pub struct DiGraph {
    assoc: Vec<Vec<usize>>,
}

impl DiGraph {
    pub fn new() -> Self {
        Self {
            assoc: vec![],
        }
    }

    pub fn len(&self) -> usize {
        self.assoc.len()
    }

    pub fn node(&mut self) -> Node {
        let index = self.assoc.len();
        self.assoc.push(vec![]);
        Node(index)
    }

    pub fn edge(&mut self, from: Node, to: Node) -> Edge {
        let from = from.0;
        let to = to.0;
        self.assoc[from].push(to);
        Edge(from, to)
    }

    pub fn has_edge(&self, from: Node, to: Node) -> bool {
        let (from, to) = (from.0, to.0);
        self.assoc[from].contains(&to)
    }

    pub fn get_node(&self, index: usize) -> Option<Node> {
        if index < self.len() {
            Some(Node(index))
        } else {
            None
        }
    }

    pub fn get_edge(&self, from: usize, to: usize) -> Option<Edge> {
        if match (self.get_node(from), self.get_node(to)) {
            (Some(from), Some(to)) => self.has_edge(from, to),
            _ => false,
        } {
            Some(Edge(from, to))
        } else {
            None
        }
    }

    pub fn cycle_from(&self, target: Node) -> bool {
        let target = target.0;
        let mut visited = vec![false; self.len()];
        let mut stack = vec![target];
        while let Some(current) = stack.pop() {
            visited[current] = true;
            for node in &self.assoc[current] {
                if !visited[*node] {
                    stack.push(*node);
                } else if *node == target {
                    return true
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cycle_from() {
        let mut graph = DiGraph::new();
        let n0 = graph.node();
        let n1 = graph.node();
        graph.edge(n0, n1);
        graph.edge(n1, n0);
        assert!(graph.cycle_from(n0));
        assert!(graph.cycle_from(n1));
    }

    #[test]
    fn no_cycle_from() {
        let mut graph = DiGraph::new();
        let n0 = graph.node();
        let n1 = graph.node();
        graph.edge(n0, n1);
        assert!(!graph.cycle_from(n0));
        assert!(!graph.cycle_from(n1));
    }

    #[test]
    fn cycle_not_containing_target() {
        let mut graph = DiGraph::new();
        let n0 = graph.node();
        let n1 = graph.node();
        let n2 = graph.node();
        graph.edge(n0, n1);
        graph.edge(n1, n0);
        graph.edge(n2, n0);
        graph.edge(n2, n1);
        assert!(graph.cycle_from(n0));
        assert!(graph.cycle_from(n1));
        assert!(!graph.cycle_from(n2));
    }

    #[test]
    fn get_node() {
        let mut graph = DiGraph::new();
        let n0 = graph.node();
        let n1 = graph.node();
        let n2 = graph.node();
        assert_eq!(graph.get_node(0), Some(n0));
        assert_eq!(graph.get_node(1), Some(n1));
        assert_eq!(graph.get_node(2), Some(n2));
        assert_eq!(graph.get_node(3), None);
    }

    #[test]
    fn get_edge() {
        let mut graph = DiGraph::new();
        let n0 = graph.node();
        let n1 = graph.node();
        let e0 = graph.edge(n0, n1);
        assert_eq!(graph.get_edge(0, 1), Some(e0));
        assert_eq!(graph.get_edge(1, 0), None);
        assert_eq!(graph.get_edge(1, 2), None);
    }
}
