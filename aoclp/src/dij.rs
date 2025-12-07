use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub trait Graph<T> {
    fn neighbours(&self, node: &T) -> impl Iterator<Item = T>;
    fn dist(&self, a: &T, b: &T) -> usize {
        let (_, _) = (a, b);
        1
    }
}

pub struct Output<T> {
    pub dist: HashMap<T, usize>,
    pub prev: HashMap<T, T>,
}

pub fn build<T, G>(graph: &G, start: T) -> Output<T>
where
    T: Clone + Eq + Hash,
    G: Graph<T>,
{
    // https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm

    let mut dist = HashMap::new();
    let mut prev = HashMap::new();
    dist.insert(start.clone(), 0);

    let mut q = HashSet::new();
    q.insert(start.clone());

    while !q.is_empty() {
        let u = q
            .iter()
            .map(|n| (n.clone(), dist.get(n).copied().unwrap_or(usize::MAX)))
            .min_by_key(|(_, u_dist)| *u_dist);
        match u {
            None => break,
            Some((u, u_dist)) => {
                q.remove(&u);
                graph.neighbours(&u).for_each(|v| {
                    let alt = u_dist + graph.dist(&u, &v);
                    match dist.entry(v.clone()) {
                        Entry::Vacant(e) => {
                            q.insert(v.clone());
                            e.insert(alt);
                            prev.insert(v, u.clone());
                        },
                        Entry::Occupied(mut e) if alt < *e.get() => {
                            e.insert(alt);
                            prev.insert(v, u.clone());
                        },
                        _ => (),
                    }
                });
            },
        }
    }

    Output { dist, prev }
}

pub fn assemble_path<T>(prev: &HashMap<T, T>, start: &T, end: &T) -> impl Iterator<Item = T>
where
    T: Clone + Eq + Hash,
{
    let mut path = Vec::new();
    let mut n = end.clone();
    while n != *start {
        path.push(n.clone());
        n = match prev.get(&n) {
            Some(p) => p.clone(),
            None => break,
        }
    }
    path.push(start.clone());
    path.into_iter().rev()
}
