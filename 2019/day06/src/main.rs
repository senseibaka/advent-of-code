use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::Iterator;
use std::collections::HashMap;
use fast_paths::InputGraph;

fn main() {
    let input = file_to_vec("input.txt".to_string()).unwrap();
    let mut g = GraphWrapper::new();
    for line in input {
        let split: Vec<_> = line.split(")").collect();
        let from = split[0];
        let to = split[1];
        g.add_edge(from.to_string(), to.to_string());
    }
    let part_1 = g.solve_part_1();
    println!("PART 1 OUTPUT: {}", part_1);
    let part_2 = g.solve_part_2();
    println!("PART 2 OUTPUT: {}", part_2);
}

struct GraphWrapper {
    graph: InputGraph,
    names: HashMap<String, usize>,
    next_id: usize
}

impl GraphWrapper {
    fn new() -> GraphWrapper {
        GraphWrapper {
            graph: InputGraph::new(),
            names: HashMap::new(),
            next_id: 0
        }
    }

    fn add_edge(&mut self, from: String, to: String) {
        let from_id = self.ensure_node(&from);
        let to_id = self.ensure_node(&to);
        //println!("{}({}) -> {}({})", &from, from_id, &to, to_id);
        self.graph.add_edge_bidir(from_id, to_id, 1);
    }

    fn ensure_node(&mut self, name: &String) -> usize {
        match self.names.get(name) {
            Some(&id) => id,
            _ => {
                let id = self.next_id;
                self.next_id += 1;
                self.names.insert(name.clone(), id);
                id
            }
        }
    }

    fn solve_part_1(&mut self) -> usize {
        self.graph.freeze();
        let fast_graph = fast_paths::prepare(&self.graph);
        let com_id: usize = self.ensure_node(&"COM".to_string());
        //println!("COM {}", com_id);
        
        //part 1
        let total_distance: usize = self.names
            .iter()
            .filter(|&(k, v)| *v != com_id)
            .map(|(k, v)| {
                let shortest_path = fast_paths::calc_path(&fast_graph, com_id, *v).unwrap();
                let dist = shortest_path.get_weight();
                //println!("DIST COM -> {} = {}", k, dist);
                dist
            })
            .fold(0, |acc, v| acc + v);
        
        self.graph.thaw();
        return total_distance;
    }

    fn solve_part_2(&mut self) -> usize {
        self.graph.freeze();
        let fast_graph = fast_paths::prepare(&self.graph);
        let you_id: usize = self.ensure_node(&"YOU".to_string());
        let san_id: usize = self.ensure_node(&"SAN".to_string());
        //println!("YOU {}, SAN {}", you_id, san_id);
        
        //part 2
        let you_to_san_path = fast_paths::calc_path(&fast_graph, you_id, san_id).unwrap();
        let dist = you_to_san_path.get_weight();

        self.graph.thaw();
        //println!("{:?}", you_to_san_path);
        return dist - 2;
    }
}

fn file_to_vec(filename: String) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_part_1_works() {
        let mut g = GraphWrapper::new();
        
        g.add_edge("COM".to_string(), "B".to_string());
        g.add_edge("B".to_string(), "C".to_string());
        g.add_edge("C".to_string(), "D".to_string());
        g.add_edge("D".to_string(), "E".to_string());
        g.add_edge("E".to_string(), "F".to_string());
        g.add_edge("B".to_string(), "G".to_string());
        g.add_edge("G".to_string(), "H".to_string());
        g.add_edge("D".to_string(), "I".to_string());
        g.add_edge("E".to_string(), "J".to_string());
        g.add_edge("J".to_string(), "K".to_string());
        g.add_edge("K".to_string(), "L".to_string());
        
        let part_1 = g.solve_part_1();

        assert_eq!(42, part_1);
    }

    #[test]
    fn solve_part_2_works() {
        let mut g = GraphWrapper::new();
        g.add_edge("COM".to_string(),"B".to_string());
        g.add_edge("B".to_string(),"C".to_string());
        g.add_edge("C".to_string(),"D".to_string());
        g.add_edge("D".to_string(),"E".to_string());
        g.add_edge("E".to_string(),"F".to_string());
        g.add_edge("B".to_string(),"G".to_string());
        g.add_edge("G".to_string(),"H".to_string());
        g.add_edge("D".to_string(),"I".to_string());
        g.add_edge("E".to_string(),"J".to_string());
        g.add_edge("J".to_string(),"K".to_string());
        g.add_edge("K".to_string(),"L".to_string());
        g.add_edge("K".to_string(),"YOU".to_string());
        g.add_edge("I".to_string(),"SAN".to_string());
        let part_2 = g.solve_part_2();

        assert_eq!(4, part_2);
    }
}