pub mod sorting {
    /// Quick sort.
    /// Adapted from the C++ implementation at: https://www.geeksforgeeks.org/quick-sort/
    pub fn quick_sort<T: PartialOrd + Clone>(v: &mut [T]) -> () {
        match v.is_empty() {
            true => (),
            false => quick_sort_impl(v, 0usize, v.len() - 1),
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
            let partition_left = partition_index.checked_sub(1);
            let partition_right = partition_index.checked_add(1); 

            quick_sort_impl(v, low, partition_left.unwrap_or(low));
            quick_sort_impl(v, partition_right.unwrap_or(high), high);
        }
    }

    pub fn quick_sort_dual_pivot<T: PartialOrd + Clone>(v: &mut [T]) -> () {
        match v.is_empty() {
            true => (),
            false => quick_sort_dual_pivot_impl(v, 0usize, v.len() - 1),
        }
    }

    /// Dual Pivot Quick-Sort
    /// Adapted from the C code at: https://www.geeksforgeeks.org/dual-pivot-quicksort/
    fn quick_sort_dual_pivot_impl<T: PartialOrd + Clone>(
        v: &mut [T],
        low: usize,
        high: usize,
    ) -> () {
        if low < high {
            let (left_partition, right_partition) = dual_pivot_partition(v, low, high);
            quick_sort_dual_pivot_impl(v, low, left_partition - 1);
            quick_sort_dual_pivot_impl(v, left_partition + 1, right_partition - 1);
            quick_sort_dual_pivot_impl(v, right_partition + 1, high);
        }
    }

    fn dual_pivot_partition<T: PartialOrd + Clone>(
        v: &mut [T],
        low: usize,
        high: usize,
    ) -> (usize, usize) {
        if v[low] > v[high] {
            v.swap(low, high);
        }
        // PIVOTS
        let left_pivot = v[low].clone();
        let right_pivot = v[high].clone();
        // RUNNING CURSORS
        let mut right_cursor = high - 1;
        let mut cursor = low + 1;
        let mut left_cursor = low + 1;
        // LOOP
        while cursor <= right_cursor {
            // MOVE ELEMENTS TO THE LEFT PARTITION
            if v[cursor] < left_pivot {
                v.swap(left_cursor, cursor);
                left_cursor += 1;
            } else if v[cursor] >= right_pivot {
                // POSITION RIGHT CURSOR TO THE FIRST ELEMENT FROM THE RIGHT LESS OR EQUAL THAN RIGHT PIVOT
                while v[right_cursor] > right_pivot && cursor < right_cursor {
                    right_cursor -= 1;
                }
                // PUT ELEMENT IN THE RIGHT PARTITON
                v.swap(cursor, right_cursor);
                // NEW RIGHT CURSOR
                right_cursor -= 1;
                // UNDER CURSOR THERE IS NOW SOMETHING LESS THAN OR EQUAL RIGHT PIVOT
                // IF IT IS ALSO LESS THAN LEFT PIVOT, SWAP
                if v[cursor] < left_pivot {
                    v.swap(cursor, left_cursor);
                    left_cursor += 1;
                }
            }
            cursor += 1;
        }
        // MOVE PARTITION CURSORS TO ACTUAL PIVOT POSITIONS
        left_cursor -= 1;
        right_cursor += 1;
        // BRING PIVOTS TO THEIR APPROPRIATE POSITIONS
        v.swap(low, left_cursor);
        v.swap(high, right_cursor);

        return (cursor, right_cursor);
    }
}

#[cfg(test)]
mod tests {
    use crate::quick_sort::sorting::{quick_sort, quick_sort_dual_pivot};

    #[test]
    fn quick_sort_empty() {
        let mut v_empty: Vec<i32> = Vec::new();

        quick_sort(&mut v_empty);

        assert_eq!(v_empty.len(), 0);
    }

    #[test]
    fn quick_sort_1() {
        let mut v: Vec<i32> = vec![1, 9, 2, 5, 4];

        quick_sort(&mut v);

        assert_eq!(v, vec![1, 2, 4, 5, 9]);
    }

    #[test]
    fn quick_sort_2() {
        let mut v: Vec<u32> = vec![10, 7, 8, 9, 1, 5];

        quick_sort(&mut v);

        assert_eq!(v, vec![1, 5, 7, 8, 9, 10]);
    }

    #[test]
    fn quick_sort_3() {
        let mut v: Vec<u32> = vec![15, 85, 32, 4, 21, 24];

        quick_sort(&mut v);

        assert_eq!(v, vec![4, 15, 21, 24, 32, 85]);
    }

    #[test]
    fn quick_sort_dual_pivot_empty() {
        let mut v: Vec<u32> = vec![];

        quick_sort_dual_pivot(&mut v);

        assert_eq!(v, vec![]);
    }

    #[test]
    fn quick_sort_dual_pivot_1() {
        let mut v: Vec<u32> = vec![10, 7, 8, 9, 1, 5];

        quick_sort_dual_pivot(&mut v);

        assert_eq!(v, vec![1, 5, 7, 8, 9, 10]);
    }
}
