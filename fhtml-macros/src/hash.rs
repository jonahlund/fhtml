use core::hash::{Hash, Hasher};
use std::hash::DefaultHasher;

pub(crate) fn hash<V: Hash>(value: &V) -> usize {
    let mut hasher = DefaultHasher::new();

    Hash::hash(value, &mut hasher);
    Hasher::finish(&hasher) as usize
}
