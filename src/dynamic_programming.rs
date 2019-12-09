use std::cmp::min;

fn edit_distance_recursive_impl(s1: &[char], s2: &[char]) -> usize {
    let m = s1.len();
    let n = s2.len();

    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }

    if s1[m - 1] == s2[n - 1] {
        edit_distance_recursive_impl(&s1[..m - 1], &s2[..n - 1]);
    }

    1usize + min(
        min(
            edit_distance_recursive_impl(&s1[0..m], &s2[0..(n - 1)]),
            edit_distance_recursive_impl(&s1[0..(m - 1)], &s2[0..n]),
        ),
        edit_distance_recursive_impl(&s1[0..(m - 1)], &s2[0..(n - 1)]),
    )
}

pub fn edit_distance_recursive(s1: &str, s2: &str) -> usize {
    let s1_slice: Vec<char> = s1.chars().collect();
    let s2_slice: Vec<char> = s2.chars().collect();

    edit_distance_recursive_impl(&s1_slice, &s2_slice)
}

fn edit_distance_dp_impl(s1: &[char], s2: &[char]) -> usize {
    let m = s1.len();
    let n = s2.len();

    let mut d: Vec<Vec<usize>> = vec![vec![0usize; n + 1]; m + 1];

    for i in 0..=m {
        for j in 0..=n {
            if i == 0 {
                d[i][j] = j;
            } else if j == 0 {
                d[i][j] = i;
            } else if s1[i - 1] == s2[j - 1] {
                d[i][j] = d[i - 1][j - 1];
            } else {
                d[i][j] = 1 + min(
                    min(
                        d[i][j - 1],
                        d[i - 1][j]
                    ),
                    d[i - 1][j - 1]
                );
            }
        }
    }

    println!("{:?}", d);

    d[m][n]
}

pub fn edit_distance_dp(s1: &str, s2: &str) -> usize {
    let s1_slice: Vec<char> = s1.chars().collect();
    let s2_slice: Vec<char> = s2.chars().collect();

    edit_distance_dp_impl(&s1_slice, &s2_slice)
}

#[cfg(test)]
mod tests {
    use crate::dynamic_programming::{edit_distance_dp, edit_distance_recursive};

    #[test]
    fn test_edit_distance_dp_01() {
        let s1 = "sunday";
        let s2 = "saturday";

        assert_eq!(edit_distance_dp(s1, s2), 3);
    }

    #[test]
    fn test_edit_distance_recursive_01() {
        let s1 = "sunday";
        let s2 = "saturday";

        assert_eq!(edit_distance_recursive(s1, s2), 3);
    }
}