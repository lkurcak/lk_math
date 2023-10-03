pub trait LinearIndex<I> {
    fn index_unchecked(&self, i: I) -> Option<usize>;
    fn unindex(&self, i: usize) -> Option<I>;
    fn is_in_bounds(&self, i: &I) -> bool;

    /// NOTE(lubo): Really, this is unsafe and should not be called as overflows are not checked. They could be, but they aren't. (They use `Iterator::product`) They are not used anywhere internally.
    #[deprecated]
    unsafe fn cardinality(&self) -> Option<usize>;

    fn index(&self, i: I) -> Option<usize> {
        if self.is_in_bounds(&i) {
            self.index_unchecked(i)
        } else {
            None
        }
    }
}
