pub trait IterUtils: IntoIterator + Sized {
    fn collect_hash_set(self) -> std::collections::HashSet<Self::Item>
    where
        Self::Item: Eq +  std::hash::Hash,
    {
        self.into_iter().collect()
    }

    fn collect_vec(self) -> Vec<Self::Item> {
        self.into_iter().collect()
    }

    fn collect_string(self, sep: &str) -> String
    where
        Self::Item: std::fmt::Display,
    {
        self.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(sep)
    }
}

impl<T: IntoIterator + Sized> IterUtils for T {}