use std::collections::VecDeque;

pub type NodeIndex = usize;
pub type EdgeIndex = usize;

#[derive(Debug)]
pub struct EdgeData {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>,
    weight: i32,
}

#[derive(Debug)]
pub struct NodeData {
    first_outgoing_edge: Option<EdgeIndex>,
    pub name: String,
    pub weight: i32,
}

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn find_node(&self, name: &str) -> Option<NodeIndex> {
        for i in 0..self.nodes.len() {
            if self.nodes[i].name == *name {
                return Some(i);
            }
        }
        None
    }

    pub fn add_unique_node(
        &mut self,
        name: String,
        weight: i32,
    ) -> Result<NodeIndex, &'static str> {
        match self.find_node(&name) {
            Some(idx) => Ok(idx),
            None => Ok(self.add_node(name, weight)),
        }
    }

    fn add_node(&mut self, name: String, weight: i32) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(NodeData {
            first_outgoing_edge: None,
            name: name,
            weight: weight,
        });
        return index;
    }
    fn add_edge(&mut self, source: NodeIndex, target: NodeIndex, weight: i32) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(EdgeData {
            target: target,
            weight: weight,
            next_outgoing_edge: node_data.first_outgoing_edge,
        });

        node_data.first_outgoing_edge = Some(edge_index);
    }

    pub fn add_edge_by_name(
        &mut self,
        source: &str,
        target: &str,
        weight: i32,
    ) -> Result<(), &'static str> {
        let source_idx = match self.find_node(source) {
            Some(idx) => Ok(idx),
            None => Err("Node not found"),
        }?;

        let target_idx = match self.find_node(target) {
            Some(idx) => Ok(idx),
            None => Err("Node not found"),
        }?;
        self.add_edge(source_idx, target_idx, weight);
        Ok(())
    }

    pub fn calculate_cost(&self, elem: &str) -> Result<i32, &'static str> {
        let mut start_node = match self.find_node(elem) {
            Some(i) => Ok(i),
            None => Err("Node not found"),
        }?;

        let mut node_vec = VecDeque::new();

        node_vec.push_back(start_node);

        loop {
            let node_idx = node_vec[0];
            node_vec.pop_front().unwrap();
            println!(
                "Node: {} {}",
                self.nodes[node_idx].name, self.nodes[node_idx].weight
            );
            // Check if node is leaf or not
            match self.nodes[node_idx].first_outgoing_edge {
                // Not leaf
                Some(i) => {
                    // Check if there are more edges for a given nodes
                    println!(
                        "\tEdge: {} {}",
                        self.nodes[self.edges[i].target].name, self.edges[i].weight
                    );

                    let mut edge_idx = i;
                    node_vec.push_back(self.edges[edge_idx].target);
                    loop {
                        match self.edges[edge_idx].next_outgoing_edge {
                            // Outgoing Edge
                            Some(i) => {
                                edge_idx = i;

                                node_vec.push_back(self.edges[edge_idx].target);
                                println!(
                                    "\tEdge: {} {}",
                                    self.nodes[self.edges[edge_idx].target].name,
                                    self.edges[edge_idx].weight
                                );
                            }
                            // No More Outgoing edges
                            None => {
                                break;
                            }
                        }
                    }
                }

                // Leaf
                None => {}
            }
            if node_vec.len() == 0 {
                break;
            }
        }
        Ok(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_unique_node() {
        let mut test_graph = Graph::new();
        assert_eq!(0, test_graph.add_unique_node(String::from("A"), 1).unwrap());
        assert_eq!(0, test_graph.add_unique_node(String::from("A"), 1).unwrap());
    }

    #[test]
    fn test_find_node() {
        let mut test_graph = Graph::new();
        test_graph.add_unique_node(String::from("A"), 1).unwrap();
        test_graph.add_unique_node(String::from("B"), 1).unwrap();
        test_graph.add_unique_node(String::from("C"), 1).unwrap();

        assert_eq!(2, test_graph.find_node("C").unwrap());
    }

    #[test]
    fn test_add_edge_by_name() {
        let mut test_graph = Graph::new();

        test_graph.add_unique_node(String::from("A"), 1).unwrap();
        test_graph.add_unique_node(String::from("B"), 1).unwrap();

        assert_eq!(Ok(()), test_graph.add_edge_by_name("A", "B", 0));
        assert_eq!(
            Err("Node not found"),
            test_graph.add_edge_by_name("A", "C", 0)
        );
    }
}
