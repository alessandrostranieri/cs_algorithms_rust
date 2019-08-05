pub mod sorting {

    pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) -> () {
        let mut v_len = v.len();
        if v.is_empty() || v_len == 1 {
            return;
        }
        let mut is_sorted = false;
        while !is_sorted {
            is_sorted = true;
            for i in 0..v_len - 1 {
                if v[i] > v[i + 1] {
                    is_sorted = false;
                    v.swap(i, i + 1);
                }
            }
            v_len = v_len - 1;
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::bubble_sort::sorting::bubble_sort;

    #[test]
    fn sort_empty_vector_does_not_change_the_vector() {
        let mut v_empty: Vec<i32> = Vec::new();

        bubble_sort(&mut v_empty);

        assert_eq!(v_empty.len(), 0);
    }

    #[test]
    fn sort_vector_produces_expected_result() {
        let mut v_filled: Vec<i32> = vec![1, 9, 2, 5, 4];

        bubble_sort(&mut v_filled);

        assert_eq!(v_filled, vec![1, 2, 4, 5, 9]);
    }
}
