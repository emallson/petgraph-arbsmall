extern crate petgraph;
extern crate quickcheck;
extern crate rand;

use petgraph::prelude::*;
use quickcheck::*;
use std::ops::Range;
use rand::distributions::range::Range as RandRange;
use rand::distributions::Sample;

pub fn arbitrary<G: Gen, N: Arbitrary, E: Arbitrary>(g: &mut G, size: Range<usize>) -> Graph<N, E> {
    let mut size_dist = RandRange::new(size.start, size.end);
    let size = size_dist.sample(g);
    let mut uniform = RandRange::new(0.0, 1.0);
    let density = uniform.sample(g);
    let mut gr = Graph::new();
    let mut nodes = Vec::with_capacity(size as usize);
    for _ in 0..size {
        nodes.push(gr.add_node(Arbitrary::arbitrary(g)));
    }

    for &u in nodes.iter() {
        for &v in nodes.iter() {
            if uniform.sample(g) <= density {
                gr.add_edge(u, v, Arbitrary::arbitrary(g));
            }
        }
    }

    return gr;
}

pub fn shrink<N: Clone + 'static, E: Clone + 'static>(g: &Graph<N, E>)
                                                      -> Box<Iterator<Item = Graph<N, E>>> {
    let other: Graph<N, E> = g.clone();
    Box::new(g.node_indices().map(move |idx| {
        let mut m = other.clone();
        m.remove_node(idx);
        m
    }))
}
