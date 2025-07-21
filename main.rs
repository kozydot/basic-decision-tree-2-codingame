use std::io;
use std::collections::HashMap;
use itertools::Itertools;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

// holds a pupa's species and all its feature values.
#[derive(Debug, Clone)]
struct Pupa {
    species: i32,
    features: Vec<i32>,
}

// calculates shannon entropy for a group of pupae.
fn calculate_set_entropy(indices: &[usize], all_pupae: &Vec<Pupa>) -> f64 {
    // an empty set has zero entropy.
    if indices.is_empty() {
        return 0.0;
    }

    // count up how many of each species we have here.
    let mut species_counts = HashMap::new();
    for &idx in indices {
        *species_counts.entry(all_pupae[idx].species).or_insert(0) += 1;
    }

    let total = indices.len() as f64;
    let mut entropy = 0.0;

    // now do the math for entropy: -Σ(p * log2(p)).
    for count in species_counts.values() {
        let p = (*count as f64) / total;
        // log2(0) is undefined, so we skip if p is 0.
        if p > 0.0 {
            entropy -= p * p.log2();
        }
    }
    entropy
}

// recursively finds the best splits to build a tree, returns the final entropy.
fn calculate_tree_leaves_entropy(
    current_indices: &[usize],
    available_features: &Vec<i32>, // these are 1-based feature ids
    all_pupae: &Vec<Pupa>,
    total_pupa_count: f64,
) -> f64 {
    let current_entropy = calculate_set_entropy(current_indices, all_pupae);

    // base case: if the group is already pure (entropy is 0), we're done with this branch.
    if current_entropy == 0.0 {
        return 0.0;
    }

    let mut min_split_entropy = current_entropy;
    let mut best_split: Option<(Vec<usize>, Vec<usize>)> = None;

    // now let's hunt for the best possible split.
    for &feature_id in available_features {
        let feature_idx_0based = (feature_id - 1) as usize;
        // any pupa in the current group could be our splitter.
        for &splitter_pupa_idx in current_indices {
            let split_value = all_pupae[splitter_pupa_idx].features[feature_idx_0based];

            let mut left_group = Vec::new();
            let mut right_group = Vec::new();

            // split the current group into two based on the feature value.
            for &pupa_idx in current_indices {
                if all_pupae[pupa_idx].features[feature_idx_0based] < split_value {
                    left_group.push(pupa_idx);
                } else {
                    right_group.push(pupa_idx);
                }
            }
            
            // a split is useless if it doesn't actually separate things.
            if left_group.is_empty() || right_group.is_empty() {
                continue;
            }

            // calculate the weighted entropy of the two new groups.
            let h_left = calculate_set_entropy(&left_group, all_pupae);
            let h_right = calculate_set_entropy(&right_group, all_pupae);

            let total_in_node = current_indices.len() as f64;
            let weighted_entropy =
                (left_group.len() as f64 / total_in_node) * h_left +
                (right_group.len() as f64 / total_in_node) * h_right;

            // if this split is better than what we've seen, save it.
            if weighted_entropy < min_split_entropy {
                min_split_entropy = weighted_entropy;
                best_split = Some((left_group, right_group));
            }
        }
    }

    // okay, we've checked all possible splits. what's the verdict?
    if let Some((left, right)) = best_split {
        // if we found a good split, recurse on the new smaller groups.
        // the total entropy is the sum from the sub-trees.
        calculate_tree_leaves_entropy(&left, available_features, all_pupae, total_pupa_count)
        + calculate_tree_leaves_entropy(&right, available_features, all_pupae, total_pupa_count)
    } else {
        // if no split helped, this group becomes a leaf node.
        // calculate its final entropy contribution: H(S) * |S| / |S_total|.
        let current_size = current_indices.len() as f64;
        current_entropy * (current_size / total_pupa_count)
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let pn = parse_input!(input_line, i32);

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let fn_val = parse_input!(input_line, i32);

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let fm_val = parse_input!(input_line, i32);

    let mut all_pupae: Vec<Pupa> = Vec::with_capacity(pn as usize);
    for _ in 0..pn {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs: Vec<i32> = input_line.split_whitespace()
                                       .map(|s| s.parse().unwrap())
                                       .collect();
        // inputs[0] is index (unused), inputs[1] is species, inputs[2..] are features.
        let pupa = Pupa {
            species: inputs[1],
            features: inputs[2..].to_vec(),
        };
        all_pupae.push(pupa);
    }
    
    // start with all pupae in one big group (indices 0 to n-1).
    let initial_indices: Vec<usize> = (0..pn as usize).collect();

    let mut min_entropy = f64::MAX;
    let mut best_features: Vec<i32> = vec![];

    // generate all feature ids we can choose from, e.g., [1, 2, 3, ...].
    let feature_ids: Vec<i32> = (1..=fn_val).collect();

    // now we test every combination of features.
    for feature_combination in feature_ids.into_iter().combinations(fm_val as usize) {
        
        // for this combo, find the best possible tree and its entropy.
        let entropy = calculate_tree_leaves_entropy(
            &initial_indices,
            &feature_combination,
            &all_pupae,
            pn as f64
        );

        // if this combo is better, it's our new winner.
        if entropy < min_entropy {
            min_entropy = entropy;
            best_features = feature_combination;
        }
        // itertools::combinations is cool because it gives us combos in sorted order.
        // this means the first time we find the minimum entropy, we've also
        // satisfied the tie-breaker rule (use smaller feature indices).
    }

    // spit out the winning feature ids, space-separated.
    // they're already sorted thanks to itertools.
    println!("{}", best_features.iter().map(i32::to_string).collect::<Vec<String>>().join(" "));
}
