mod graph;
mod helpers;
mod ops;
mod small_array;

trait Value: Default + Copy {}
impl<V: Default + Copy> Value for V {}

trait Position: Default + Copy + Into<usize> {}
impl<P: Default + Copy + Into<usize>> Position for P {}

#[cfg(test)]
mod test_fixtures {
    use crate::{
        graph::{Edges, Nodes},
        helpers::{edge, edges, nodes, weighted_edge},
    };

    pub(crate) fn sample_nodes() -> Nodes<i32> {
        println!("test");
        nodes(vec![0, 0, 0, 0])
    }

    pub(crate) fn sample_edges() -> Edges<usize, i32> {
        edges(vec![
            vec![edge(0, 1), edge(0, 2)],
            vec![edge(1, 2)],
            vec![edge(2, 0)],
            vec![],
        ])
    }

    pub(crate) fn sample_weighted_edges() -> Edges<usize, i32> {
        edges(vec![
            vec![weighted_edge(0, 1, vec![1]), weighted_edge(0, 2, vec![100])],
            vec![weighted_edge(1, 2, vec![1])],
            vec![weighted_edge(2, 0, vec![2])],
            vec![],
        ])
    }
}
