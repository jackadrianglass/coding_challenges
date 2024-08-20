pub type VertexHandle = usize;
pub type EdgeHandle = (usize, usize);

pub trait Graph<'a, Vertex: 'a, Edge: 'a> {
    fn new() -> Self;

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
}
