/// Copyright 2025 Simon De Ridder
/// This file is part of FriendVote.
/// FriendVote is free software: you can redistribute it and/or modify it under the terms of the
/// GNU General Public License as published by the Free Software Foundation, either version 3 of the License,
/// or (at your option) any later version.
/// FriendVote is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
/// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
/// See the GNU General Public License for more details.
/// You should have received a copy of the GNU General Public License along with FriendVote.
/// If not, see <https://www.gnu.org/licenses/>.
#[cfg(feature = "ssr")]
/// Tideman's ranked pairs (winning variant)
pub fn calculate_ranks(defeats_matrix: &Vec<Vec<Option<u64>>>) -> std::collections::HashMap<usize, usize> {
	//(Vec<Vec<bool>>, Vec<Vec<usize>>) {

	// get sorted vec of defeats

	use std::collections::{HashMap, HashSet};

	use petgraph::{graph::NodeIndex, visit::EdgeRef};
	let mut defeats = Vec::<(usize, usize, u64, u64)>::new();
	{
		for row_ind in 0..defeats_matrix.len() {
			for col_ind in 0..row_ind {
				if defeats_matrix[row_ind][col_ind] > defeats_matrix[col_ind][row_ind] {
					defeats.push((
						row_ind,
						col_ind,
						defeats_matrix[row_ind][col_ind].unwrap(),
						defeats_matrix[col_ind][row_ind].unwrap(),
					));
				} else if defeats_matrix[row_ind][col_ind] < defeats_matrix[col_ind][row_ind] {
					defeats.push((
						col_ind,
						row_ind,
						defeats_matrix[col_ind][row_ind].unwrap(),
						defeats_matrix[row_ind][col_ind].unwrap(),
					));
				}
			}
		}
		let max_defeat_majority = defeats.iter().map(|d| d.2).max().unwrap();
		defeats.sort_by_key(|defeat| (max_defeat_majority - defeat.2, defeat.3));
	}

	// group defeats by strength
	let mut defeats_grouped_ind = Vec::new();
	{
		let mut prev_key = (defeats[0].2, defeats[0].3);
		let mut defeats_group_ind = Vec::new();
		for (defeat_ind, defeat) in defeats.iter().enumerate() {
			if (defeat.2, defeat.3) != prev_key {
				defeats_grouped_ind.push(defeats_group_ind.clone());
				defeats_group_ind = Vec::new();
				defeats_group_ind.push(defeat_ind);
				prev_key = (defeat.2, defeat.3);
			} else {
				defeats_group_ind.push(defeat_ind);
			}
		}
		if !defeats_group_ind.is_empty() {
			defeats_grouped_ind.push(defeats_group_ind.clone());
		}
	}

	// create graph
	let mut defeat_graph = petgraph::stable_graph::StableDiGraph::<usize, ()>::new();
	let mut nodes = Vec::with_capacity(defeats_matrix.len());
	{
		for candidate_index in 0usize..defeats_matrix.len() {
			let node_ind = defeat_graph.add_node(candidate_index);
			nodes.push(node_ind);
		}
	}
	let mut ac_defeat_graph = petgraph::acyclic::Acyclic::try_from_graph(defeat_graph).unwrap();
	{
		let mut new_edges = Vec::new();
		let mut defeat;
		let mut cycle_found = false;
		for defeats_group in defeats_grouped_ind {
			new_edges.clear();
			for defeat_ind in defeats_group.iter() {
				defeat = defeats.get(*defeat_ind).unwrap();
				match ac_defeat_graph.try_add_edge(nodes[defeat.0], nodes[defeat.1], ()) {
					Ok(edge_ind) => new_edges.push(edge_ind),
					Err(_) => {
						// cycle found, abort adding this defeats group
						for edge_id in new_edges.iter() {
							ac_defeat_graph.remove_edge(*edge_id);
						}
						cycle_found = true;
						break;
					},
				};
			}
			if cycle_found {
				break;
			}
		}
	}

	// find node ranks
	let mut node_ranks = nodes
		.iter()
		.map(|n| (*n, nodes.len()))
		.collect::<HashMap<NodeIndex, usize>>();
	{
		let mut current_layer = 0;
		let mut current_layer_nodes = HashSet::new();
		for node in nodes.iter() {
			if ac_defeat_graph.edges_directed(*node, petgraph::Incoming).count() == 0 {
				current_layer_nodes.insert(*node);
			}
		}
		let mut next_layer_nodes;
		while !current_layer_nodes.is_empty() {
			next_layer_nodes = HashSet::new();
			for node in current_layer_nodes.iter() {
				node_ranks.insert(*node, current_layer);
				for edge in ac_defeat_graph.edges_directed(*node, petgraph::Outgoing) {
					next_layer_nodes.insert(edge.target());
				}
			}
			current_layer_nodes = next_layer_nodes;
			current_layer += 1;
		}
	}

	node_ranks
		.iter()
		.map(|(node_id, rank)| (*ac_defeat_graph.node_weight(*node_id).unwrap(), *rank))
		.collect()
}
