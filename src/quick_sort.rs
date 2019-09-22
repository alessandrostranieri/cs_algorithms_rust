pub mod sorting {
    /// Quick sort.
    /// Adapted from the C++ implementation at: https://www.geeksforgeeks.org/quick-sort/
    pub fn quick_sort<T: PartialOrd + Clone>(v: &mut [T]) -> () {
        match v.is_empty() {
            true => (),
            false => quick_sort_impl(v, 0usize, v.len() - 1)
        }
    }

    fn partition<T: PartialOrd + Clone>(v: &mut [T], low: usize, high: usize) -> usize {
        // SELECTS PIVOT
        let pivot = v[high].clone();
        let mut low_index = low;

        for j in low..high {
            if v[j] < pivot {
                v.swap(low_index, j);
                low_index += 1;
            }
        }
        // PUT THE PIVOT AT THE RIGHT PLACE
        v.swap(low_index, high);
        return low_index;
    }

    fn quick_sort_impl<T: PartialOrd + Clone>(v: &mut [T], low: usize, high: usize) -> () {
        if low < high {
            let partition_index = partition(v, low, high);

            quick_sort_impl(v, low, partition_index - 1);
            quick_sort_impl(v, partition_index + 1, high);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::quick_sort::sorting::quick_sort;

    #[test]
    fn sort_empty_vector_does_not_change_the_vector() {
        let mut v_empty: Vec<i32> = Vec::new();

        quick_sort(&mut v_empty);

        assert_eq!(v_empty.len(), 0);
    }

    #[test]
    fn sort_vector_1() {
        let mut v: Vec<i32> = vec![1, 9, 2, 5, 4];

        quick_sort(&mut v);

        assert_eq!(v, vec![1, 2, 4, 5, 9]);
    }

    #[test]
    fn sort_vector_2() {
        let mut v: Vec<u32> = vec![10, 7, 8, 9, 1, 5];

        quick_sort(&mut v);

        assert_eq!(v, vec![1, 5, 7, 8, 9, 10]);
    }
}
