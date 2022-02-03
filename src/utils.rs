use std::cmp::Ordering;

/// Return the index of a sorted slice
///
/// # Arguments
///
/// * `data` - The slice that should be sorted by the index that is returned.
///
pub fn argsort<T: PartialOrd>(data: &[T]) -> Vec<usize> {
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_by(|a_idx, b_idx| {
        reverse_ordering(
            data[*a_idx]
                .partial_cmp(&data[*b_idx])
                .unwrap_or(Ordering::Less),
        )
    });
    indices
}
/// Reverse ordering
///
/// # Arguments
///
/// * `ordering` - The current ordering that needs to be reversed.
///
fn reverse_ordering(ordering: Ordering) -> Ordering {
    match ordering {
        Ordering::Greater => Ordering::Less,
        Ordering::Less => Ordering::Greater,
        Ordering::Equal => Ordering::Equal,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod test_argsort {
        use super::*;
        #[test]
        fn four_floats() {
            assert_eq!(argsort(&vec![1.0, 5.0, 3.0, 6.0]), vec![3, 1, 2, 0]);
        }
        #[test]
        fn thirteen_floats() {
            assert_eq!(
                argsort(&vec![
                    13.0, 14.0, 12.0, 10.0, 22.0, 6.0, 16.0, 24.0, 18.0, 23.0, 15.0, 11.0, 17.0
                ]),
                vec![7, 9, 4, 8, 12, 6, 10, 1, 0, 2, 11, 3, 5],
            );
        }

        #[test]
        fn five_isize() {
            assert_eq!(argsort(&vec![2, 5, 3, 4, 1, 6]), vec![5, 1, 3, 2, 0, 4]);
        }
    }
    mod test_reverse_ordering {
        use super::*;
        #[test]
        fn greater_to_less() {
            assert_eq!(reverse_ordering(Ordering::Greater), Ordering::Less)
        }
        #[test]
        fn less_to_greater() {
            assert_eq!(reverse_ordering(Ordering::Less), Ordering::Greater)
        }
        #[test]
        fn equal_stays() {
            assert_eq!(reverse_ordering(Ordering::Equal), Ordering::Equal)
        }
    }
}
