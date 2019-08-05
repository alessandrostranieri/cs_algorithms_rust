mod search {
    pub fn binary_search<T: PartialOrd>(nums: &Vec<T>, target: T) -> Option<usize> {
        if nums.is_empty() {
            return None;
        }

        let mut start = 0usize;
        let mut end = nums.len() - 1;

        if target < nums[start] || target > nums[end] {
            return None;
        }

        while start <= end {
            let middle = (end + start) / 2;
            if target == nums[middle] {
                return Some(middle);
            } else if target < nums[middle] {
                end = middle - 1;
            } else if target > nums[middle] {
                start = middle + 1;
            }
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_search::search::binary_search;

    #[test]
    fn test_for_empty_vector_return_none() {
        assert_eq!(binary_search(&vec![], 42), Option::None);
    }

    #[test]
    fn test_successful_search_odd() {
        assert_eq!(binary_search(&vec![1, 42, 100], 42), Some(1));
    }

    #[test]
    fn test_successful_search_even() {
        assert_eq!(binary_search(&vec![1, 42, 69, 100], 42), Some(1));
    }

    #[test]
    fn test_successful_start() {
        assert_eq!(binary_search(&vec![1, 42, 69, 100], 1), Some(0));
    }

    #[test]
    fn test_successful_end() {
        assert_eq!(binary_search(&vec![1, 42, 69, 100], 100), Some(3));
    }

    #[test]
    fn test_unsuccessful_even() {
        assert_eq!(binary_search(&vec![1, 42, 69, 100], 50), None);
    }

    #[test]
    fn test_unsuccessful_odd() {
        assert_eq!(binary_search(&vec![1, 42, 100], 50), None);
    }

    #[test]
    fn test_unsuccessful_start() {
        assert_eq!(binary_search(&vec![1, 42, 69, 100], 0), None);
    }

    #[test]
    fn test_unsuccessful_end() {
        assert_eq!(binary_search(&vec![1, 42, 69, 100], 101), None);
    }
}