/// Chop as vector into `count` chunks, returning an iterator
pub(crate) fn chop<'a, A: Clone>(v: &'a Vec<A>, count: usize) -> impl Iterator<Item = Vec<A>> + 'a {
    let size = chunk_size(v.len(), count);
    v.chunks(size).map(|c| c.to_vec())
}

fn chunk_size(length: usize, count: usize) -> usize {
    let q = length / count;
    let r = length % count;
    if r > 0 {q + 1} else {q}
}

/// Apply a function to both components of a pair
pub(crate) fn both<'a, A, B>(f: impl Fn(&'a A) -> B, pair: &'a (A, A)) -> (B, B) {
    (f(&pair.0), f(&pair.1))
}

/// Zip a pair of iterators, yielding a iterator of pairs
/// zip_pair :: ([a], [b]) -> [(a,b)]
pub(crate) fn zip_pair<'a, A, B>(
    pair: (impl Iterator<Item = A> + 'a, impl Iterator<Item = B> + 'a),
) -> impl Iterator<Item = (A, B)> + 'a {
    pair.0.zip(pair.1)
}
