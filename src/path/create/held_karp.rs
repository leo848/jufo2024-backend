use std::{fmt::Debug, fs::OpenOptions, hint::black_box, iter::empty, ops::Range};

use itertools::Itertools;

use crate::{
    dist_graph::Point,
    graph::Path,
    path::{CreateContext, Matrix},
    typed::Metric,
    Graph,
};

#[derive(Copy, Clone)]
struct MaskSet {
    inner: usize,
}

impl MaskSet {
    fn from_usize(value: usize) -> Self {
        Self { inner: value }
    }

    fn from_k(k: usize) -> Self {
        Self { inner: 1 << k }
    }

    fn from_range(range: Range<usize>) -> Self {
        Self {
            inner: {
                let mut n = 1;
                for _ in range.clone().skip(1) {
                    n <<= 1;
                    n += 1;
                }
                n << range.start
            },
        }
    }

    fn is_empty(self) -> bool {
        self.inner == 0
    }

    fn without(self, k: usize) -> Self {
        Self {
            inner: self.inner & !(1 << k),
        }
    }

    fn with(self, k: usize) -> Self {
        Self {
            inner: self.inner | (1 << k),
        }
    }

    fn contains(self, k: usize) -> bool {
        self.inner & (1 << k) != 0
    }

    fn subsets(range: Range<usize>) -> impl Iterator<Item = Self> {
        let Range { start, end } = range;
        (0usize..1 << end - start)
            .map(move |n| n << start)
            .map(Self::from_usize)
    }

    fn subsets_sized(range: Range<usize>, cardinality: usize) -> impl Iterator<Item = Self> {
        Self::subsets(range).filter(move |s| s.cardinality() == cardinality)
    }

    fn cardinality(self) -> usize {
        self.inner.count_ones() as usize
    }

    fn iter(self) -> impl Iterator<Item = usize> {
        (0..64).filter(move |&k| self.contains(k))
    }
}

impl IntoIterator for MaskSet {
    type Item = usize;
    type IntoIter = MaskSetIter;

    fn into_iter(self) -> Self::IntoIter {
        MaskSetIter::new(self)
    }
}

struct MaskSetIter {
    counter: usize,
    set: MaskSet,
}

impl MaskSetIter {
    fn new(set: MaskSet) -> Self {
        Self { counter: 0, set }
    }
}

impl Iterator for MaskSetIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter > 63 {
            None
        } else if self.set.contains(self.counter) {
            self.counter += 1;
            Some(self.counter - 1)
        } else {
            self.counter += 1;
            self.next()
        }
    }
}

impl Debug for MaskSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", "{", self.iter().join(", "), "}")
    }
}

struct HeldKarpDpCache {
    /// [[f32; 2^size]; size]
    /// dp_memo[k][set] = C(set, k)
    /// Memoisierung: Wichtigstes Element der dynamischen Programmierung.
    /// Speichert für (set, k) die Länge des Pfads, der bei k endet und alle Elemente von set
    /// besucht.
    c_cache: Box<[Box<[f32]>]>,
    /// Speichert für (set, x) den vorherigen (p für *p*revious) Knoten vom Pfad über alle Elemente von set über
    /// x.
    p_cache: Box<[Box<[u8]>]>,
}

impl HeldKarpDpCache {
    fn new(size: usize) -> Self {
        Self {
            c_cache: vec![vec![f32::MAX; 1 << size].into_boxed_slice(); size].into_boxed_slice(),
            p_cache: vec![vec![205; 1 << size].into_boxed_slice(); size].into_boxed_slice(),
        }
    }

    #[cfg(debug_assertions)]
    fn p(&self, subset: MaskSet, k: usize) -> u8 {
        let value = self.p_cache[k][subset.inner];
        if value == 205 {
            panic!("tried to access uninitialized ({subset:?} {k})")
        } else {
            value
        }
    }

    #[cfg(not(debug_assertions))]
    fn p(&self, subset: MaskSet, k: usize) -> u8 {
        self.p_cache[k][subset.inner]
    }

    #[cfg(debug_assertions)]
    fn c(&self, subset: MaskSet, k: usize) -> f32 {
        let value = self.c_cache[k][subset.inner];
        if value == f32::MAX {
            panic!("tried to access uninitialized ({subset:?} {k})")
        } else {
            value
        }
    }

    #[cfg(not(debug_assertions))]
    fn c(&self, subset: MaskSet, k: usize) -> f32 {
        self.c_cache[k][subset.inner]
    }

    fn set(&mut self, subset: MaskSet, k: usize, c: f32, p: u8) {
        self.c_cache[k][subset.inner] = c;
        self.p_cache[k][subset.inner] = p;
    }
}

pub fn solve<C: CreateContext>(ctx: C) -> C::Path {
    use std::collections::{HashMap, HashSet};

    let n = ctx.len();

    assert!(n < 32);

    let mut global_best_path = None;
    let mut global_best_chain_len = f32::INFINITY;

    for start_point in 0..n {
        let local_ctx = ctx.clone().rotate_left(start_point);

        let dist = local_ctx.adjacency_matrix();

        let mut cache: HeldKarpDpCache = HeldKarpDpCache::new(n);

        for k in 1..n {
            cache.set(MaskSet::from_k(k), k, dist[(0, k)], k as u8);
        }

        for subset_size in 2..n {
            for subset in MaskSet::subsets_sized(1..n, subset_size) {
                for k in subset {
                    let mut minimum = f32::INFINITY;
                    let mut min_prev = 204; // Sentinelwert: 204 = 0xCC
                    for m in subset {
                        if m == 0 || m == k {
                            continue;
                        }
                        let value = cache.c(subset.without(k), m) + dist[(m, k)];
                        if value <= minimum {
                            minimum = value;
                            min_prev = m;
                        }
                    }

                    cache.set(subset, k, minimum, min_prev as u8);
                }
            }
        }

        let mut minimum_chain_len = f32::INFINITY;
        let mut parent = 0;
        for k in 1..n {
            let chain_len_k = cache.c(MaskSet::from_range(1..n), k);
            if chain_len_k < minimum_chain_len {
                minimum_chain_len = chain_len_k;
                parent = k;
                cache.set(MaskSet::from_range(1..n), 0, minimum_chain_len, k as u8);
            }
        }

        if minimum_chain_len < global_best_chain_len {
            global_best_chain_len = minimum_chain_len;

            let mut path = Vec::new();
            let mut bits = MaskSet::from_range(1..n);

            for _ in 0..n - 1 {
                path.push(parent);
                let new_bits = bits.without(parent);
                parent = cache.p(bits, parent).into();
                bits = new_bits;
            }

            path.push(0);

            let path = path.into_iter().rev();

            let global_path = path.map(|e| (e + start_point) % n);

            global_best_path = Some(global_path.collect_vec());
        }
        ctx.send_path(
            global_best_path.iter().flatten().copied(),
            Some(start_point as f32 / n as f32),
        )
    }

    match global_best_path {
        Some(path) => ctx.path_from_indices(path.iter().copied()),
        None => panic!("No path"),
    }
}
