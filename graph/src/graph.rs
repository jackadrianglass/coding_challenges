pub type VertexHandle = usize;
pub type EdgeHandle = (usize, usize);

pub trait Graph<'a, Vertex: 'a, Edge: 'a>
where
    Vertex: Clone,
{
    fn num_vertices(&self) -> usize;
    fn vertex(&self, handle: VertexHandle) -> Option<&Vertex>;
    fn add_vertex(&mut self, data: Vertex) -> VertexHandle;
    fn add_connected_vertex(
        &mut self,
        vertex_data: Vertex,
        connecting_vertex: VertexHandle,
        edge_data: Edge,
    ) -> Option<(VertexHandle, EdgeHandle)>;
    fn remove_vertex(&mut self, handle: VertexHandle) -> Option<Vertex>;
    fn update_vertex(&mut self, handle: VertexHandle, data: Vertex);

    fn num_edges(&self) -> usize;
    fn edge(&self, handle: EdgeHandle) -> Option<&Edge>;
    fn add_edge(
        &mut self,
        first: VertexHandle,
        second: VertexHandle,
        data: Edge,
    ) -> Option<EdgeHandle>;
    fn remove_edge(&mut self, handle: EdgeHandle) -> Option<Edge>;
    fn update_edge(&mut self, handle: EdgeHandle, data: Edge);
    fn connections(&self, handle: VertexHandle) -> Vec<(&EdgeHandle, &Edge)>;
}

pub struct GraphIter<'a, Vertex, Edge> {
    graph: &'a dyn Graph<'a, Vertex, Edge>,
    start: VertexHandle,
    edge_callback: Box<dyn Fn(&[(&EdgeHandle, &Edge)]) -> Option<EdgeHandle> + 'a>,

    current: Option<VertexHandle>,
}

impl<'a, Vertex, Edge> GraphIter<'a, Vertex, Edge> {
    pub fn new(
        graph: &'a dyn Graph<'a, Vertex, Edge>,
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

impl<'a, Vertex, Edge> Iterator for GraphIter<'a, Vertex, Edge>
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
    use crate::undirected_graph::UndirectedGraph;

    #[test]
    fn can_traverse_the_graph() {
        let mut graph: UndirectedGraph<i32, i32> = UndirectedGraph::new();
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
