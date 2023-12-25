pub fn p1(input: &str) -> String {
    let graph = parse_input(input);

    let l = laplacean(graph);
    let (_, eigvecs) = l.eigh(ndarray_linalg::UPLO::Lower).unwrap();
    let fiedler = eigvecs.column(1);

    let n_positive = fiedler.into_iter().filter(|v| **v > 0.).count();
    let n_negative = fiedler.len() - n_positive;

    format!("Product: {}", n_positive * n_negative)
}

pub fn p2(input: &str) -> String {
    p1(input)
}

fn parse_input(input: &str) -> Graph {
    let mut adjacency_map: HashMap<_, _> = input
        .trim()
        .lines()
        .map(|line| {
            let (key, val) = line.split_once(":").unwrap();
            let neighbors = val.split_whitespace().map(|s| s.to_string()).collect();
            (key.to_string(), neighbors)
        })
        .collect();

    // Make symmetric
    for (key, neighbors) in adjacency_map.clone().into_iter() {
        for neighbor in neighbors {
            adjacency_map
                .entry(neighbor)
                .or_insert_with(Vec::new)
                .push(key.clone())
        }
    }

    adjacency_map
}

fn laplacean(graph: Graph) -> Array2<f64> {
    let node_ids: HashMap<String, usize> = graph
        .keys()
        .enumerate()
        .map(|(idx, key)| (key.clone(), idx))
        .collect();

    let mut adj_mx = Array2::zeros((node_ids.len(), node_ids.len()));

    for (node, neighbors) in graph.iter() {
        let node_id = node_ids.get(node).unwrap();
        for neighbor in neighbors {
            let neighbor_id = node_ids.get(neighbor).unwrap();

            *adj_mx.get_mut((*node_id, *neighbor_id)).unwrap() = 1.;
        }
    }

    let deg_mx = Array2::from_diag(&adj_mx.sum_axis(Axis(0)));

    assert_eq!(deg_mx.shape(), adj_mx.shape());

    deg_mx - adj_mx
}

type Graph = HashMap<String, Vec<String>>;

use std::collections::HashMap;

use ndarray::{Array2, Axis};
use ndarray_linalg::Eigh;

use crate::solution::Solution;
inventory::submit!(Solution::new(25, 1, p1));
inventory::submit!(Solution::new(25, 2, p2));
