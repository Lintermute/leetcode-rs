//! [LeetCode problem 2467: Most Profitable Path in a Tree][1]
//!
//! [1]: https://leetcode.com/problems/most-profitable-path-in-a-tree

use std::{
    cmp::{self, Ordering},
    collections::{HashMap, HashSet},
};

pub fn most_profitable_path(
    edges: Vec<Vec<i32>>,
    bob: i32,
    amount: Vec<i32>,
) -> i32 {
    let bob = usize::try_from(bob).unwrap();
    let (children, parents) = make_tree(edges);
    let node_to_steps = walk_bob(&parents, bob);
    let no_children = HashSet::new();

    let mut max = i32::MIN;
    let mut stack = vec![(0, 0, 0)];
    while let Some((node, depth, income)) = stack.pop() {
        let amount = match node_to_steps.get(&node) {
            None => amount[node],
            Some(steps) => match depth.cmp(steps) {
                Ordering::Less => amount[node],
                Ordering::Equal => amount[node] / 2,
                Ordering::Greater => 0,
            },
        };

        let next_depth = depth + 1;
        let curr_income = income + amount;
        let children = children
            .get(&node)
            .unwrap_or(&no_children);

        if children.is_empty() {
            max = cmp::max(max, curr_income);
        } else {
            stack.extend(
                children
                    .iter()
                    .map(|&child| (child, next_depth, curr_income)),
            );
        }
    }

    max
}

fn make_tree(
    edges: Vec<Vec<i32>>,
) -> (HashMap<usize, HashSet<usize>>, HashMap<usize, usize>) {
    let edges = edges.into_iter().flat_map(|pair| {
        let pair: [i32; 2] = pair.try_into().unwrap();
        let [from, dest] = pair.map(|node| usize::try_from(node).unwrap());
        [(from, dest), (dest, from)]
    });

    let mut bidir: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (a, b) in edges {
        if let Some(partners) = bidir.get_mut(&a) {
            partners.insert(b);
        } else {
            bidir.insert(a, HashSet::from([b]));
        }
    }

    let mut children: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut parents: HashMap<usize, usize> = HashMap::new();
    let mut processed = HashSet::new();
    let mut stack = vec![0];
    while let Some(node) = stack.pop() {
        let next = bidir
            .get(&node)
            .unwrap()
            .iter()
            .filter(|&p| !processed.contains(p));

        for &child in next {
            parents.insert(child, node);
            if let Some(children) = children.get_mut(&node) {
                children.insert(child);
            } else {
                children.insert(node, HashSet::from([child]));
            }
            stack.push(child);
        }
        processed.insert(node);
    }

    (children, parents)
}

fn walk_bob(
    parents: &HashMap<usize, usize>,
    bob: usize,
) -> HashMap<usize, usize> {
    let mut steps = HashMap::from([(bob, 0)]);
    let mut stack = vec![(bob, 0)];
    while let Some((node, depth)) = stack.pop() {
        let Some(&parent) = parents.get(&node) else {
            break;
        };

        let depth = depth + 1;
        steps.insert(parent, depth);
        stack.push((parent, depth));
    }

    steps
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case(
       &[&[0,1],&[1,2],&[1,3],&[3,4]],
       3,
       &[-2,4,2,-4,6],
       6
    )]
    #[test_case(
       &[&[0,1]],
       1,
       &[-7280,2350],
       -7280
    )]
    #[test_case(
       &[&[0,1],&[1,2],&[2,3]],
       3,
       &[-5644,-6018,1188,-8502],
       -11662
    )]
    #[test_case(
       &[&[0,2],&[0,5],&[1,3],&[1,5],&[2,4]],
       4,
       &[5018,8388,6224,3466,3808,3456],
       20328
    )]
    fn test(e: &[&[i32]], b: i32, a: &[i32], r: i32) {
        let e = e.iter().map(|e| e.to_vec()).collect();
        let a = a.to_vec();
        assert_eq!(super::most_profitable_path(e, b, a), r);
    }
}
