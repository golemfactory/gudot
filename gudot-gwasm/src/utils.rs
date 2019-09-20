use gmorph::Enc;

/// Chop as vector into `count` chunks, returning an iterator
pub(crate) fn chop<'a>(v: &'a Vec<Enc>, count: usize) -> impl Iterator<Item = Vec<Enc>> + 'a {
    v.chunks(v.len() / count).map(|c| c.to_vec())
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
