mod grandiso {

    use petgraph::graphmap::{DiGraphMap, NodeTrait};
    use std::collections::{HashMap, VecDeque};
    use std::vec::Vec;

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
        T: NodeTrait,
        U: NodeTrait,
        V: NodeTrait,
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
        let most_interesting_node = interestingness
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(k, _v)| k)
            .unwrap();

        // As a result, the first step is to see if the candidate mapping is empty.
        if candidate.is_empty() {
            // If it is, then our first step is to pick a node in the motif graph and
            // tentatively assign it to every node in the host graph.
            let mut next_candidates = Vec::<HashMap<T, V>>::new();
            for u in host.nodes() {
                let mut candidate = HashMap::new();
                candidate.insert(*most_interesting_node, u);
                next_candidates.push(candidate);
            }
            return next_candidates;
            // most_interesting_node
        }

        // Practically:
        // Given your Most Interesting Node, assign it to all
        // possible options in the host graph.

        return Vec::<HashMap<T, V>>::new();
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
        T: NodeTrait,
        U: NodeTrait,
        V: NodeTrait,
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
    fn it_works() {
        let mut graphmap: DiGraphMap<&str, &str> = DiGraphMap::new();
        graphmap.add_node("A");
        // graphmap.add_node("B");
        // graphmap.add_edge("A", "B", "");

        assert_eq!(
            grandiso::find_motifs(graphmap.clone(), graphmap.clone()).len(),
            1
        )
    }
}
