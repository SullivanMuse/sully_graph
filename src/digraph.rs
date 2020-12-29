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
}
