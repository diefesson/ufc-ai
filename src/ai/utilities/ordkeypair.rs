pub struct OrdKeyPair<K: Ord, V>(pub K, pub V);

impl<K: Ord, V> OrdKeyPair<K, V> {
    pub fn new(k: K, v: V) -> Self {
        Self(k, v)
    }
}

impl<K: Ord, V> PartialEq for OrdKeyPair<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<K: Ord, V> Eq for OrdKeyPair<K, V> {}

impl<K: Ord, V> PartialOrd for OrdKeyPair<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<K: Ord, V> Ord for OrdKeyPair<K, V> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
