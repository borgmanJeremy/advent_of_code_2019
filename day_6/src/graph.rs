// Based on http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/
pub type NodeIndex = usize;
pub type EdgeIndex = usize;

#[derive(Debug)]
pub struct EdgeData {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>,
}

#[derive(Debug)]
pub struct NodeData {
    first_outgoing_edge: Option<EdgeIndex>,
    pub name: String,
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<NodeData>,
    pub edges: Vec<EdgeData>,
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

    pub fn add_unique_node(&mut self, name: String) -> Result<NodeIndex, &'static str> {
        match self.find_node(&name) {
            Some(idx) => Ok(idx),
            None => Ok(self.add_node(name)),
        }
    }

    pub fn add_node(&mut self, name: String) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(NodeData {
            first_outgoing_edge: None,
            name: name,
        });
        return index;
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(EdgeData {
            target: target,
            next_outgoing_edge: node_data.first_outgoing_edge,
        });

        node_data.first_outgoing_edge = Some(edge_index);
    }

    pub fn add_edge_by_name(&mut self, source: &str, target: &str) -> Result<(), &'static str> {
        let source_idx = match self.find_node(source) {
            Some(idx) => Ok(idx),
            None => Err("Node not found"),
        }?;

        let target_idx = match self.find_node(target) {
            Some(idx) => Ok(idx),
            None => Err("Node not found"),
        }?;
        self.add_edge(source_idx, target_idx);
        Ok(())
    }

    pub fn route_to_head(&self, elem : &str) -> Result<Vec<String>, &'static str>
    {
        let mut node_idx = match self.find_node(elem) {
            Some(i) => Ok(i),
            None => Err("Node not found"),
        }?;
        let mut path =Vec::new();
        loop {
            match self.nodes[node_idx].first_outgoing_edge {
                Some(i) => {
                    path.push(self.nodes[node_idx].name.clone());
                    node_idx = self.edges[i].target;
                }
                None => {
                    path.push(self.nodes[node_idx].name.clone());
                    break;
                }
            }
        }
        Ok(path)
    }

    pub fn least_common_ancestor(&self, elem_1: &str, elem_2 : &str)->Result<String, &'static str> {
        let route_1 = self.route_to_head(elem_1)?;
        let route_2 = self.route_to_head(elem_2)?;
        
        let common_ancestor : String;
        for hop in &route_1{
            for i in 0..route_2.len(){
                if *hop == route_2[i] {
                    common_ancestor = hop.clone();
                    return Ok(common_ancestor);
                }
            }
        }
        Err("No common node found")
    }

    pub fn count_to_node(&self, elem_1: &str, elem_2 : &str) ->Result<i32, &'static str>{
        let mut start_node_idx = match self.find_node(elem_1) {
            Some(i) => Ok(i),
            None => Err("Node not found"),
        }?;
        let mut count: i32 = 0;
        loop {
            match self.nodes[start_node_idx].first_outgoing_edge {
                Some(i) => {
                    if self.nodes[start_node_idx].name == elem_2{
                        return Ok(count);
                    }

                    count += 1;
                    start_node_idx = self.edges[i].target;
                }
                None => {
                    break;
                }
            }
        }
        Ok(count)
    }

    pub fn count_to_head(&self, elem: &str) -> Result<i32, &'static str> {
        let mut node_idx = match self.find_node(elem) {
            Some(i) => Ok(i),
            None => Err("Node not found"),
        }?;

        let mut count: i32 = 0;
        loop {
            match self.nodes[node_idx].first_outgoing_edge {
                Some(i) => {
                    count += 1;
                    node_idx = self.edges[i].target;
                }
                None => {
                    break;
                }
            }
        }
        Ok(count)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_to_node() -> Result<(), &'static str> {
        let mut test_graph = Graph::new();
        test_graph.add_unique_node(String::from("A"))?;
        test_graph.add_unique_node(String::from("B"))?;
        test_graph.add_unique_node(String::from("C"))?;
        test_graph.add_unique_node(String::from("D"))?;
        test_graph.add_unique_node(String::from("E"))?;
       

        test_graph.add_edge_by_name("E", "C")?;
        test_graph.add_edge_by_name("D", "C")?;
        test_graph.add_edge_by_name("C", "B")?;
        test_graph.add_edge_by_name("B", "A")?;

        assert_eq!(1,test_graph.count_to_node("E", "C").unwrap());
        Ok(())
    }

    #[test]
    fn test_least_common_ancestor() -> Result<(), &'static str> {
        let mut test_graph = Graph::new();
        test_graph.add_unique_node(String::from("A"))?;
        test_graph.add_unique_node(String::from("B"))?;
        test_graph.add_unique_node(String::from("C"))?;
        test_graph.add_unique_node(String::from("D"))?;
        test_graph.add_unique_node(String::from("E"))?;
       

        test_graph.add_edge_by_name("E", "C")?;
        test_graph.add_edge_by_name("D", "C")?;
        test_graph.add_edge_by_name("C", "B")?;
        test_graph.add_edge_by_name("B", "A")?;

        assert_eq!("C",test_graph.least_common_ancestor("E", "D").unwrap());
        Ok(())
    }
    
    #[test]
    fn test_route_to_head()  -> Result<(), &'static str> {
        let mut test_graph = Graph::new();
        test_graph.add_unique_node(String::from("A"))?;
        test_graph.add_unique_node(String::from("B"))?;
        test_graph.add_unique_node(String::from("C"))?;
        test_graph.add_unique_node(String::from("D"))?;
        test_graph.add_unique_node(String::from("E"))?;
       

        test_graph.add_edge_by_name("E", "C")?;
        test_graph.add_edge_by_name("D", "C")?;
        test_graph.add_edge_by_name("C", "B")?;
        test_graph.add_edge_by_name("B", "A")?;

        let route = test_graph.route_to_head("E").unwrap();

        assert_eq!(vec!["E","C","B","A"],route);
        Ok(())
    }

    #[test]
    fn test_count_to_head() -> Result<(), &'static str> {
        let mut test_graph = Graph::new();
        test_graph.add_unique_node(String::from("A"))?;
        test_graph.add_unique_node(String::from("B"))?;
        test_graph.add_unique_node(String::from("C"))?;
        test_graph.add_unique_node(String::from("D"))?;

        test_graph.add_edge_by_name("D", "C")?;
        test_graph.add_edge_by_name("C", "B")?;
        test_graph.add_edge_by_name("B", "A")?;

        assert_eq!(0, test_graph.count_to_head("A").unwrap());
        assert_eq!(1, test_graph.count_to_head("B").unwrap());
        assert_eq!(2, test_graph.count_to_head("C").unwrap());
        assert_eq!(3, test_graph.count_to_head("D").unwrap());

        Ok(())
    }
    #[test]
    fn test_add_unique_node() {
        let mut test_graph = Graph::new();
        assert_eq!(0, test_graph.add_unique_node(String::from("A")).unwrap());
        assert_eq!(0, test_graph.add_unique_node(String::from("A")).unwrap());
    }

    #[test]
    fn test_find_node() {
        let mut test_graph = Graph::new();
        test_graph.add_unique_node(String::from("A")).unwrap();
        test_graph.add_unique_node(String::from("B")).unwrap();
        test_graph.add_unique_node(String::from("C")).unwrap();

        assert_eq!(2, test_graph.find_node("C").unwrap());
    }

    #[test]
    fn test_add_edge_by_name() {
        let mut test_graph = Graph::new();

        test_graph.add_unique_node(String::from("A")).unwrap();
        test_graph.add_unique_node(String::from("B")).unwrap();

        assert_eq!(Ok(()), test_graph.add_edge_by_name("A", "B"));
        assert_eq!(Err("Node not found"), test_graph.add_edge_by_name("A", "C"));
    }
}
