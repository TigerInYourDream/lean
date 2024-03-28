use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

pub struct GraphAdjList<T> {
    adj_list: HashMap<T, Vec<T>>,
}

impl<T: Hash + PartialEq + Eq + Copy> GraphAdjList<T> {
    fn new(edges: Vec<[T; 2]>) -> Self {
        let mut graph = Self {
            adj_list: HashMap::new(),
        };
        for edge in edges {
            graph.add_vertex(edge[0]);
            graph.add_vertex(edge[1]);
            graph.add_edge(edge[0], edge[1]);
        }
        graph
    }

    pub fn add_vertex(&mut self, vertex: T) {
        if self.adj_list.contains_key(&vertex) {
            return;
        }
        self.adj_list.insert(vertex, vec![]);
    }

    pub fn add_edge(&mut self, vertex1: T, vertex2: T) {
        if !self.adj_list.contains_key(&vertex1)
            || !self.adj_list.contains_key(&vertex2)
            || vertex1 == vertex2
        {
            panic!("value error");
        }
        self.adj_list.get_mut(&vertex1).unwrap().push(vertex2);
        self.adj_list.get_mut(&vertex2).unwrap().push(vertex1);
    }

    pub fn size(&self) -> usize {
        self.adj_list.len()
    }

    pub fn remove_edge(&mut self, vertex1: T, vertex2: T) {
        if  !self.adj_list.contains_key(&vertex1) {
            panic!("vertex1 not exist");
        }
        if !self.adj_list.contains_key(&vertex2) {
            panic!("vertex2 not exist");
        }
        if vertex1 == vertex2 {
            panic!("two vertex equals");
        }
        self.adj_list
            .get_mut(&vertex1)
            .unwrap()
            .retain(|&vet| vet != vertex2);
        self.adj_list
            .get_mut(&vertex2)
            .unwrap()
            .retain(|&vet| vet != vertex1);
    }

    pub fn remove_vertex(&mut self, vertex: T) {
        if !self.adj_list.contains_key(&vertex) {
            panic!("vertex not exist");
        }

        self.adj_list.remove(&vertex);
        // delete in chain (edge in other vec)
        for list in self.adj_list.values_mut() {
            list.retain(|&vt| vt != vertex);
        }
    }
}

pub fn gragh_bfs<T>(graph: GraphAdjList<T>, start_vertex: T) -> Vec<T>
where
    T: Hash + PartialEq + Copy + Eq,
{
    let mut res = vec![];
    let mut visted = HashSet::new();
    visted.insert(start_vertex);

    let mut queue = VecDeque::new();
    queue.push_back(start_vertex);

    while !queue.is_empty() {
        let vertex = queue.pop_front().unwrap();
        res.push(vertex);

        if let Some(adj_vets) = graph.adj_list.get(&vertex) {
            for &adj_vet in adj_vets {
                if visted.contains(&adj_vet) {
                    continue;
                }
                queue.push_back(adj_vet);
                visted.insert(adj_vet);
            }
        }
    }

    res
}

pub fn graph_dfs<T>(graph: GraphAdjList<T>, start_vertex: T) -> Vec<T>
where
    T: Hash + PartialEq + Copy + Eq,
{
    let mut res = vec![];
    let mut visted = HashSet::new();
    dfs(&graph, &mut visted, &mut res, start_vertex);
    res
}

fn dfs<T>(graph: &GraphAdjList<T>, visted: &mut HashSet<T>, res: &mut Vec<T>, vertex: T)
where
    T: Hash + PartialEq + Copy + Eq,
{
    res.push(vertex);
    visted.insert(vertex);

    if let Some(adj_vets) = graph.adj_list.get(&vertex) {
        for &adj_vet in adj_vets {
            if visted.contains(&adj_vet) {
                continue;
            }
            dfs(graph, visted, res, adj_vet);
        }
    }
}
