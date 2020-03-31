pub unsafe trait NonOverlappingMapIterator<'a, V> {
    type Item;
    fn next_mapped(&mut self, values: &'a mut [V]) -> Option<Self::Item>;
}

pub struct NonOverlappingMapIteratorDriver<'a, V, I>
where
    V: 'a,
{
    values: &'a mut [V],
    inner_iter: I,
}

impl<'a, V, I> NonOverlappingMapIteratorDriver<'a, V, I>
where
    V: 'a,
    I: NonOverlappingMapIterator<'a, V>,
{
    pub fn map_values(values: &'a mut [V], inner_iter: I) -> Self {
        let values = values.into();
        Self { values, inner_iter }
    }
}

impl<'a, V, I> Iterator for NonOverlappingMapIteratorDriver<'a, V, I>
where
    V: 'a,
    I: NonOverlappingMapIterator<'a, V>,
{
    type Item = <I as NonOverlappingMapIterator<'a, V>>::Item;

    // Some unsafe magic is required here.
    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next_mapped(self.values)
    }
}

unsafe impl<'a, V> NonOverlappingMapIterator<'a, V> for std::ops::Range<usize>
where
    V: 'a,
{
    type Item = &'a mut V;

    fn next_mapped(&mut self, values: &'a mut [V]) -> Option<Self::Item> {
        self.next().map(|idx| values.get_mut(idx).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut values = vec![1, 2, 3];
        let values_slice = values.as_mut_slice();
        let inner_iter = 0..values.len();

        let iter = NonOverlappingMapIteratorDriver::map_values(values_slice, inner_iter);

        let val = iter.next().unwrap();
        assert_eq!(*val, 1);
        *val = 10;

        let val = iter.next().unwrap();
        assert_eq!(*val, 2);
        *val = 20;

        let val = iter.next().unwrap();
        assert_eq!(*val, 3);
        *val = 30;

        assert!(iter.next().is_none());

        drop(iter);

        assert_eq!(values, &[10, 20, 30]);
    }
}
