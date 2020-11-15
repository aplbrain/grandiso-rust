pub mod grandiso {

    use petgraph::graphmap::{DiGraphMap, NodeTrait};
    use std::vec::Vec;
    use std::{
        collections::{HashMap, HashSet, VecDeque},
        fmt::Debug,
    };

    fn _is_structural_match<T, U, V, W>(
        _motif_node: T,
        _host_node: V,
        _candidate: HashMap<T, V>,
        _motif: &DiGraphMap<T, U>,
        _host: &DiGraphMap<V, W>,
    ) -> bool
    where
        T: NodeTrait,
        U: NodeTrait,
        V: NodeTrait,
        W: NodeTrait,
    {
        true
    }

    /// Perform a single iteration of candidate-mapping growth.
    ///
    /// # Arguments
    ///
    /// * `candidate` - The partial candidate mapping
    /// * `motif` - The motif graph network
    /// * `host` - The host graph
    /// * `interestingness` - A mapping of some search-order heuristic
    ///
    fn get_next_candidates<T, U, V, W>(
        candidate: HashMap<T, V>,
        motif: &DiGraphMap<T, U>,
        host: &DiGraphMap<V, W>,
        interestingness: &HashMap<T, f32>,
    ) -> Vec<HashMap<T, V>>
    where
        T: NodeTrait + Debug,
        U: NodeTrait,
        V: NodeTrait + Debug,
        W: NodeTrait,
    {
        // Right now we don't implement a preferred "next node" since I never
        // used that in the Python version anyway.
        // TODO: Could parametrize next_node.

        // The most interesting node is defined as the node with the maximum
        // interestingness score, which satisfies the following criteria:
        // * If the candidate is empty, then any node will do;
        // * If the candidate has a value, then the most interesting node must
        //   be connected to a node in the candidate set.

        // As a result, the first step is to see if the candidate mapping is empty.
        if candidate.is_empty() {
            // If the candidate is empty, the most interesting node is defined
            // as the node with the maximum interestingness in general
            let most_interesting_node: &T;
            most_interesting_node = interestingness
                .iter()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .map(|(k, _v)| k)
                .unwrap();

            // Our first step is to pick a node in the motif graph and
            // tentatively assign it to every node in the host graph.
            let mut next_candidates = Vec::<HashMap<T, V>>::new();
            for u in host.nodes() {
                // TODO: Filter based upon degree of the node so we're not
                // going to go crazy.
                let mut candidate = HashMap::new();
                candidate.insert(*most_interesting_node, u);
                next_candidates.push(candidate);
            }

            // For the empty candidate map case, we're done. Return the list
            // of ALL possible node mappings.
            // TODO: Filter these by degree, or by attributes, or... anything
            // other than returning ALL nodes. That's ridiculous.
            return next_candidates;
        } else {
            // The incoming mapping already has some nodes assigned. Let's pick
            // a motif node such that it connects to a node in the candidate
            // already, and such that it maximizes interestingness.
            // Note that this operation is analogous to the Python here:
            // https://github.com/aplbrain/grandiso-networkx/blob/b5db289c7b8a681776c264014ec4f31c6431d37d/grandiso/__init__.py#L158
            // However, in the Python implementation this is performed in
            // several steps. In Rust, it is all possible to roll this into one
            // big statement in order to avoid the (impossible, but Rust has no
            // way of knowing that) memory error associated with failing to
            // find a valid next most interesting node.
            // Why is this impossible? Two reasons. First, we assert that the
            // motif has only a single connected component. Second, we assert
            // that any time this function is invoked, the candidate map is
            // already-begun (i.e. nonempty) AS WELL AS incomplete (i.e. not
            // the same size as |V|-motif). In these cases, it is impossible
            // NOT to find a node that satisfies the below filtering.
            let most_interesting_node = motif
                .nodes()
                .filter(|node| !candidate.contains_key(node))
                .max_by_key(|node| {
                    // Given a node, count how many of its neighbors appear in the
                    // candidate mapping. This number must be greater than zero,
                    // assuming a motif with a single connected component.
                    return motif
                        .neighbors(*node)
                        .filter(|v| candidate.contains_key(&v))
                        .count();
                })
                .unwrap();

            // The following is the analog of the following Python:
            // https://github.com/aplbrain/grandiso-networkx/blob/b5db289c7b8a681776c264014ec4f31c6431d37d/grandiso/__init__.py#L192
            // Specifically, we now have the name of a node `most_interesting_node`
            // which we know is connected to the candidate mapping by at least
            // one edge.
            // In the next step, get a list of all such edges. We will do this
            // all in one go: For each edge, we'll first identify the motif
            // nodes in play, and then we will identify all possible nodes in
            // the host graph s.t. these edges have valid mappings.
            let mut required_edges = vec![];
            for neighboring_node in motif.neighbors(most_interesting_node) {
                if candidate.contains_key(&neighboring_node) {
                    required_edges.push((most_interesting_node, neighboring_node));
                }
                // TODO: Special treatment for directed graphs needed?
            }

            // Now we have a list of all edges that must exist given the
            // mapping so far.
            // Now we must find candidate nodes in the host graph that have the
            // edges that are required. We will store a list of such nodes in
            // a vec before returning new mappings.
            let candidate_host_nodes: Vec<V>;

            // It is impossible for required_edges to have length == 0, because
            // we DEFINED the most-interesting-node to have nonzero edges.
            // That leaves a few possibilities:
            // In the worst-case, `required_edges` has length == 1. This is the
            // worst because it rules out the fewest new nodes from the host.
            // ANY unclaimed node with a single connection to our M-I-N is a
            // valid mapping.
            if required_edges.len() == 1 {
                let (_m_i_n, neighbor) = required_edges.first().unwrap();
                candidate_host_nodes = host.neighbors(*candidate.get(neighbor).unwrap()).collect();
            } else if required_edges.len() > 1 {
                // This is the better case; it means we can easily whittle down
                // the total number of valid nodes here by checking to see if
                // they have enough valid mappings in the current candidate.
                let mut candidate_host_nodes_set = HashSet::<V>::new();
                required_edges.iter().for_each(|(_m_i_n, neighbor)| {
                    let candidate_nodes_from_this_edge =
                        host.neighbors(*candidate.get(neighbor).unwrap());

                    let candidate_nodes_from_this_edge_set: HashSet<V> =
                        candidate_nodes_from_this_edge.collect();

                    // !!!!!!!!!!!!!!!!!!!!! :242 in py
                    // Candidate host nodes are the set intersection of all
                    // previous set entries and the new entries from this edge.
                    // If the set is empty, that implies that this is the first
                    // edge that we are checking, so SET the candidate set.
                    // If there are already candidates in the set, then perform
                    // the set intersection.
                    if candidate_host_nodes_set.is_empty() {
                        candidate_host_nodes_set
                            .extend(candidate_nodes_from_this_edge_set.iter().cloned());
                    } else {
                        candidate_host_nodes_set.intersection(&candidate_nodes_from_this_edge_set);
                    }
                });
                candidate_host_nodes = candidate_host_nodes_set.iter().cloned().collect();
            }
            // py 253:
            // If len(required_edges) is 0, something bad happened. Probably
            // the motif has multiple connected components?
            else {
                panic!(
                    "Invalid motif.\
                Does it perhaps have more than one connected component?\n\
                \
                Some diagnostic information:\n \
                Required edges: {:?}\n\
                Current M-I-N: {:?}\n\

                ",
                    required_edges.len(),
                    most_interesting_node
                );
            }

            let tentative_new_candidates = candidate_host_nodes.iter().map(|candidate_node| {
                let mut new_candidate = candidate.clone();
                new_candidate.insert(most_interesting_node, *candidate_node);
                return new_candidate;
            });

            // Perform one final filtering step here:
            // We have determined that all of these NODES belong in the map,
            // but we have not yet established that all of these edges are
            // indeed supplied for this motif.
            // For example, if you have edges missing between two nodes but
            // both nodes already exist in this candidate mapping, we may have
            // reported it as "complete" even though it's not. (e.g. A,B,C, you
            // already have explored all of the nodes via edges AB and BC, but
            // have not yet verified that edge CA exists.)
            let new_monomorphism_candidates = tentative_new_candidates.filter(|candidate| {
                // If this doesn't have an assignment for each motif yet,
                // we can just return it; it's a valid mapping but incomplete.
                if candidate.len() != motif.node_count() {
                    return true;
                } else {
                    // Verify that all motif edges exist.
                    motif.all_edges().all(|(u, v, _)| {
                        if !candidate.contains_key(&u) {
                            panic!(
                                "{:#?} not in host.\
                            mapping: {:#?}\
                            host nodes: {:#?}",
                                u,
                                candidate,
                                host.nodes()
                                    .map(|n| format!("{:?}", n))
                                    .collect::<Vec<String>>()
                                    .join(",")
                            );
                        }
                        if !candidate.contains_key(&v) {
                            panic!(
                                "{:#?} not in host.\
                            mapping: {:#?}\
                            host nodes: {:#?}",
                                v,
                                candidate,
                                host.nodes()
                                    .map(|n| format!("{:?}", n))
                                    .collect::<Vec<String>>()
                                    .join(",")
                            );
                        }
                        return host.contains_edge(
                            *candidate.get(&u).unwrap(),
                            *candidate.get(&v).unwrap(),
                        );
                    })
                }
            });
            // TODO: We ignore isomorphism here.
            println!("{:?}", new_monomorphism_candidates);
            return new_monomorphism_candidates.collect();
        }
    }

    /// Identify all candidate subgraph monomorphisms between a motif and
    /// a host graph.
    ///
    /// # Arguments
    ///
    /// * `motif` - The motif graph network
    /// * `host` - The host graph
    ///
    pub fn find_motifs<T, U, V, W>(
        motif: DiGraphMap<T, U>,
        host: DiGraphMap<V, W>,
    ) -> Vec<HashMap<T, V>>
    where
        T: NodeTrait + Debug,
        U: NodeTrait,
        V: NodeTrait + Debug,
        W: NodeTrait,
    {
        // First, create the big data structures that are going to hold
        // the state-space and the results. These have short little
        // names because they won't be on the stage for that long, and
        // because we're already in Generics-hell. Glad I'm not
        // writing this in Go. (boo hiss, hot take!)

        // Create an empty list for results storage, R:
        let mut r = Vec::<HashMap<T, V>>::new();

        // Create queue Q:
        let mut q = VecDeque::<HashMap<T, V>>::new();

        // Generate a nodewise lookup (map) of interestingness.
        // For simplicity, we're just using the uniform metric.
        // TODO: Smarter interestingness heuristics!
        let mut interestingness = HashMap::new();
        motif.nodes().for_each(|f| {
            let _ = interestingness.insert(f, 1f32);
        });

        // Add to Q the the set of all mappings with one node.
        // TODO: If we instead start here with the set of all
        // edges, the total initial queue growth will be greatly
        // reduced, which is desired.
        let initial_mapping = HashMap::new();

        q.push_back(initial_mapping);

        // Now loop until the queue is empty.
        while !q.is_empty() {
            // Get a list of next valid candidate mappings:
            let next_mappings =
                get_next_candidates(q.pop_front().unwrap(), &motif, &host, &interestingness);

            next_mappings.iter().for_each(|mapping| {
                if mapping.len() == motif.node_count() {
                    // Then this is a complete mapping; add it to the set of
                    // valid result mappings.
                    r.push(mapping.clone());
                } else {
                    // Otherwise, this is a valid partial mapping, and it
                    // shoudl be added back into the queue:
                    q.push_back(mapping.clone());
                }
            })
        }

        return r;
    }
}
#[cfg(test)]
mod tests {

    use crate::grandiso;
    use petgraph::graphmap::DiGraphMap;

    #[test]
    fn test_single_node() {
        let mut graphmap: DiGraphMap<&str, &str> = DiGraphMap::new();
        graphmap.add_node("A");

        assert_eq!(
            grandiso::find_motifs(graphmap.clone(), graphmap.clone()).len(),
            1
        )
    }

    #[test]
    fn test_two_edges() {
        let mut graphmap: DiGraphMap<i8, i8> = DiGraphMap::new();
        graphmap.add_edge(0, 1, 2);
        graphmap.add_edge(1, 2, 3);

        assert_eq!(
            grandiso::find_motifs(graphmap.clone(), graphmap.clone()).len(),
            1
        )
    }

    #[test]
    fn test_directed_triangles() {
        let mut graphmap: DiGraphMap<&str, &str> = DiGraphMap::new();
        graphmap.add_edge("0", "1", "3");
        graphmap.add_edge("1", "2", "3");
        graphmap.add_edge("2", "0", "3");

        assert_eq!(
            grandiso::find_motifs(graphmap.clone(), graphmap.clone()).len(),
            1
        )
    }
}
