pub mod sorting {
    pub fn insertion_sort<T: PartialOrd>(v: &mut [T]) -> () {
        let v_len = v.len();
        if v.is_empty() || v_len == 1 {
            return;
        }

        let mut i = 1usize;
        while i < v_len {
            let mut j = i;
            while j > 0 && v[j-1] > v[j] {
                v.swap(j-1, j);
                j -= 1;
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::insertion_sort::sorting::insertion_sort;

    #[test]
    fn sort_empty_vector_does_not_change_the_vector() {
        let mut v_empty: Vec<i32> = Vec::new();

        insertion_sort(&mut v_empty);

        assert_eq!(v_empty.len(), 0);
    }

    #[test]
    fn sort_single_element_vector_does_not_change_the_vector() {
        let mut v_one= vec![42];

        insertion_sort(&mut v_one);

        assert_eq!(vec![42], v_one);
    }

    #[test]
    fn sort_vector_produces_expected_result() {
        let mut v_filled: Vec<i32> = vec![1, 9, 2, 5, 4];

        insertion_sort(&mut v_filled);

        assert_eq!(v_filled, vec![1, 2, 4, 5, 9]);
    }
}