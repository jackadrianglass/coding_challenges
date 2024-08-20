use crate::graph::*;
use std::collections::HashMap;

#[derive(Clone)]
struct DirectedGraph<Vertex, Edge> {
    vertices: HashMap<VertexHandle, Vertex>,
    edges: HashMap<EdgeHandle, Edge>,
    vertex_counter: VertexHandle,
}

impl<'a, Vertex: 'a, Edge: 'a> Graph<'a, Vertex, Edge> for DirectedGraph<Vertex, Edge> {
    fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: HashMap::new(),
            vertex_counter: 0,
        }
    }

    fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    fn vertex(&self, handle: VertexHandle) -> Option<&Vertex> {
        self.vertices.get(&handle)
    }

    fn add_vertex(&mut self, data: Vertex) -> VertexHandle {
        let id = self.vertex_counter;
        self.vertex_counter += 1;

        self.vertices.insert(id, data);

        id
    }

    fn add_connected_vertex(
        &mut self,
        vertex_data: Vertex,
        connecting_vertex: VertexHandle,
        edge_data: Edge,
    ) -> Option<(VertexHandle, EdgeHandle)> {
        let handle = self.add_vertex(vertex_data);
        let edge_handle = self.add_edge(connecting_vertex, handle, edge_data);
        match edge_handle {
            Some(edge_handle) => Some((handle, edge_handle)),
            None => {
                self.remove_vertex(handle);
                None
            }
        }
    }

    fn remove_vertex(&mut self, handle: VertexHandle) -> Option<Vertex> {
        let to_remove = self
            .edges
            .iter_mut()
            .filter(|&((a, b), _)| *a == handle || *b == handle)
            .map(|(key, _)| key.clone())
            .collect::<Vec<_>>();

        for key in to_remove.iter() {
            self.edges.remove(key);
        }

        self.vertices.remove(&handle)
    }

    fn update_vertex(&mut self, handle: VertexHandle, data: Vertex) {
        match self.vertices.get_mut(&handle) {
            Some(val) => *val = data,
            None => {}
        }
    }

    fn num_edges(&self) -> usize {
        self.edges.len()
    }

    fn edge(&self, handle: EdgeHandle) -> Option<&Edge> {
        self.edges.get(&handle)
    }

    // the edge handle returned is in the format of from -> to
    fn add_edge(
        &mut self,
        from: VertexHandle,
        to: VertexHandle,
        data: Edge,
    ) -> Option<EdgeHandle> {
        if !(self.vertices.contains_key(&from) && self.vertices.contains_key(&to)) {
            return None;
        }
        let handle = (from, to);
        if self.edges.contains_key(&handle) {
            return None;
        }
        self.edges.insert(handle, data);
        Some(handle)
    }

    fn remove_edge(&mut self, handle: EdgeHandle) -> Option<Edge> {
        self.edges.remove(&handle)
    }

    fn update_edge(&mut self, handle: EdgeHandle, data: Edge) {
        match self.edges.get_mut(&handle) {
            Some(val) => *val = data,
            None => {}
        }
    }
}

impl<Vertex, Edge> DirectedGraph<Vertex, Edge>
where
    Vertex: Clone,
{
    fn connections(&self, handle: VertexHandle) -> Vec<(&EdgeHandle, &Edge)> {
        self.edges
            .iter()
            .filter(|&((a, b), _)| *a == handle || *b == handle)
            .collect()
    }

    fn in_edges(&self, handle: VertexHandle) -> Vec<(&EdgeHandle, &Edge)> {
        self.edges
            .iter()
            .filter(|&((_, b), _)| *b == handle)
            .collect()
    }

    fn out_edges(&self, handle: VertexHandle) -> Vec<(&EdgeHandle, &Edge)> {
        self.edges
            .iter()
            .filter(|&((a, _), _)| *a == handle)
            .collect()
    }

    fn traverse<'b>(
        &'b self,
        start: VertexHandle,
        edge_callback: Box<dyn Fn(&[(&EdgeHandle, &Edge)]) -> Option<EdgeHandle>>,
    ) -> impl Iterator<Item = (VertexHandle, Vertex)> + 'b {
        DirectedGraphIter::new(&self, start, edge_callback)
    }
}

struct DirectedGraphIter<'a, Vertex, Edge> {
    graph: &'a DirectedGraph<Vertex, Edge>,
    start: VertexHandle,
    edge_callback: Box<dyn Fn(&[(&EdgeHandle, &Edge)]) -> Option<EdgeHandle> + 'a>,

    current: Option<VertexHandle>,
}

impl<'a, Vertex, Edge> DirectedGraphIter<'a, Vertex, Edge> {
    fn new(
        graph: &'a DirectedGraph<Vertex, Edge>,
        start: VertexHandle,
        edge_callback: Box<dyn Fn(&[(&EdgeHandle, &Edge)]) -> Option<EdgeHandle>>,
    ) -> Self {
        Self {
            graph,
            start,
            edge_callback,
            current: None,
        }
    }
}

impl<'a, Vertex, Edge> Iterator for DirectedGraphIter<'a, Vertex, Edge>
where
    Vertex: Clone,
{
    type Item = (VertexHandle, Vertex);

    fn next(&mut self) -> Option<Self::Item> {
        let Some(current) = self.current else {
            self.current = Some(self.start);

            if let Some(data) = self.graph.vertex(self.start) {
                return Some((self.start.clone(), data.clone()));
            } else {
                return None;
            };
        };

        if self.graph.vertex(current).is_none() {
            return None;
        }

        let edges = self.graph.connections(current);
        if edges.len() == 0 {
            return None;
        }
        let Some(edge) = (self.edge_callback)(&edges) else {
            return None;
        };
        self.current = Some(if edge.0 == current { edge.1 } else { edge.0 });
        if let Some(data) = self.graph.vertex(self.current.unwrap()) {
            Some((self.current.unwrap().clone(), data.clone()))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_vertex() {
        let mut graph: DirectedGraph<i32, i32> = DirectedGraph::new();

        assert_eq!(graph.num_vertices(), 0);
        let handle = graph.add_vertex(1);
        assert_eq!(graph.num_vertices(), 1);
        assert_eq!(graph.vertex(handle), Some(1).as_ref());
    }

    #[test]
    fn can_update_vertex() {
        let mut graph: DirectedGraph<i32, i32> = DirectedGraph::new();

        let handle = graph.add_vertex(1);
        graph.update_vertex(handle, 3);
        assert_eq!(graph.num_vertices(), 1);
        assert_eq!(graph.vertex(handle), Some(3).as_ref());
    }

    #[test]
    fn can_remove_vertex() {
        let mut graph: DirectedGraph<i32, i32> = DirectedGraph::new();

        let handle = graph.add_vertex(1);
        let vertex = graph.remove_vertex(handle);
        assert_eq!(graph.num_vertices(), 0);
        assert_eq!(vertex, Some(1));

        // trying twice does nothing
        let vertex = graph.remove_vertex(handle);
        assert_eq!(graph.num_vertices(), 0);
        assert_eq!(vertex, None);
    }

    #[test]
    fn can_add_edge() {
        let mut graph: DirectedGraph<i32, i32> = DirectedGraph::new();

        let a = graph.add_vertex(1);
        let b = graph.add_vertex(2);
        let handle = graph.add_edge(a, b, 134);

        assert_eq!(graph.num_edges(), 1);
        assert!(handle.is_some());
        assert_eq!(graph.edge(handle.unwrap()), Some(134).as_ref());

        let handle = graph.add_edge(a, b, 134);
        assert_eq!(graph.num_edges(), 1);
        assert!(handle.is_none());

        // Can add an edge in the opposite direction
        let handle = graph.add_edge(b, a, 321);
        assert_eq!(graph.num_edges(), 2);
        assert!(handle.is_some());
        assert_eq!(graph.edge(handle.unwrap()), Some(321).as_ref());
    }

    #[test]
    fn can_edit_edge() {
        let mut graph: DirectedGraph<i32, i32> = DirectedGraph::new();

        let a = graph.add_vertex(1);
        let b = graph.add_vertex(2);
        let handle = graph.add_edge(a, b, 134);
        graph.update_edge(handle.unwrap(), 321);

        assert_eq!(graph.num_edges(), 1);
        assert_eq!(graph.edge(handle.unwrap()), Some(321).as_ref());
    }

    #[test]
    fn can_remove_edge() {
        let mut graph: DirectedGraph<i32, i32> = DirectedGraph::new();

        let a = graph.add_vertex(1);
        let b = graph.add_vertex(2);
        let handle = graph.add_edge(a, b, 134);
        let data = graph.remove_edge(handle.unwrap());

        assert_eq!(graph.num_edges(), 0);
        assert_eq!(data, Some(134));
    }

    #[test]
    fn will_remove_edge_if_vertex_is_removed() {
        let mut graph: DirectedGraph<i32, i32> = DirectedGraph::new();

        let a = graph.add_vertex(1);
        let b = graph.add_vertex(2);
        let handle = graph.add_edge(a, b, 134);
        graph.remove_vertex(a);

        assert_eq!(graph.num_edges(), 0);
        assert_eq!(graph.edge(handle.unwrap()), None);
    }

    #[test]
    fn cannot_add_edge_if_vertex_doesnt_exist() {
        let mut graph: DirectedGraph<i32, i32> = DirectedGraph::new();

        let a = graph.add_vertex(1);
        let b = graph.add_vertex(2);
        graph.remove_vertex(a);
        let handle = graph.add_edge(a, b, 134);

        assert_eq!(graph.num_edges(), 0);
        assert_eq!(handle, None);
    }

    #[test]
    fn can_add_connected_vertex() {
        let mut graph: DirectedGraph<i32, i32> = DirectedGraph::new();
        let a = graph.add_vertex(1);
        let b = graph.add_connected_vertex(2, a, 123);
        assert!(b.is_some());
        assert_eq!(graph.num_vertices(), 2);
        assert_eq!(graph.num_edges(), 1);
    }

    #[test]
    fn can_get_all_connections_to_the_vertex() {
        let mut graph: DirectedGraph<i32, i32> = DirectedGraph::new();

        let a = graph.add_vertex(1);
        let b = graph.add_vertex(2);
        let c = graph.add_vertex(3);
        let d = graph.add_vertex(4);
        let e = graph.add_vertex(5);
        let handle_1 = graph.add_edge(a, b, 123);
        let handle_2 = graph.add_edge(a, c, 456);
        let handle_3 = graph.add_edge(a, d, 789);
        let handle_4 = graph.add_edge(a, e, 321);

        assert_eq!(graph.num_edges(), 4);
        let connections = graph.connections(a);
        assert_eq!(connections.len(), 4);
        let connections = connections
            .iter()
            .map(|&(key, _)| key.clone())
            .collect::<Vec<_>>();
        assert!(connections.contains(&handle_1.unwrap()));
        assert!(connections.contains(&handle_2.unwrap()));
        assert!(connections.contains(&handle_3.unwrap()));
        assert!(connections.contains(&handle_4.unwrap()));
    }

    #[test]
    fn can_get_all_input_connections_to_the_vertex() {
        let mut graph: DirectedGraph<i32, i32> = DirectedGraph::new();

        let a = graph.add_vertex(1);
        let b = graph.add_vertex(2);
        let c = graph.add_vertex(3);
        let d = graph.add_vertex(4);
        let e = graph.add_vertex(5);

        graph.add_edge(a, b, 123);
        graph.add_edge(a, c, 456);
        let handle_3 = graph.add_edge(d, a, 789);
        let handle_4 = graph.add_edge(e, a, 321);

        let connections = graph.in_edges(a);
        assert_eq!(connections.len(), 2);
        let connections = connections
            .iter()
            .map(|&(key, _)| key.clone())
            .collect::<Vec<_>>();
        assert!(connections.contains(&handle_3.unwrap()));
        assert!(connections.contains(&handle_4.unwrap()));
    }

    fn can_get_all_output_connections_to_the_vertex() {
        let mut graph: DirectedGraph<i32, i32> = DirectedGraph::new();

        let a = graph.add_vertex(1);
        let b = graph.add_vertex(2);
        let c = graph.add_vertex(3);
        let d = graph.add_vertex(4);
        let e = graph.add_vertex(5);

        let handle_1 = graph.add_edge(a, b, 123);
        let handle_2 = graph.add_edge(a, c, 456);
        graph.add_edge(d, a, 789);
        graph.add_edge(e, a, 321);

        let connections = graph.out_edges(a);
        assert_eq!(connections.len(), 2);
        let connections = connections
            .iter()
            .map(|&(key, _)| key.clone())
            .collect::<Vec<_>>();
        assert!(connections.contains(&handle_1.unwrap()));
        assert!(connections.contains(&handle_2.unwrap()));
    }

    #[test]
    fn can_traverse_the_graph() {
        let mut graph: DirectedGraph<i32, i32> = DirectedGraph::new();
        let a = graph.add_vertex(1);
        let (b, _) = graph.add_connected_vertex(2, a, 11).unwrap();
        let (c, _) = graph.add_connected_vertex(3, b, 12).unwrap();
        let (d, _) = graph.add_connected_vertex(4, c, 13).unwrap();
        let (e, _) = graph.add_connected_vertex(5, d, 14).unwrap();
        let (f, _) = graph.add_connected_vertex(6, e, 15).unwrap();

        for ((lhs, _), rhs) in graph
            .traverse(
                a,
                Box::new(|edges: &[(&EdgeHandle, &i32)]| {
                    let mut sorted = edges.iter().map(|(handle, _)| handle).collect::<Vec<_>>();
                    sorted.sort();
                    Some(***sorted.iter().rev().next().unwrap())
                }),
            )
            .zip([a, b, c, d, e, f].iter())
        {
            assert_eq!(lhs, *rhs);
        }
    }
}
