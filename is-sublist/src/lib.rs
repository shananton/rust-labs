use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Comparison {
    Equal,
    // список `a` равен списку `b`
    Sublist,
    // список `a` является подсписком `b`
    Superlist,
    // список `b` является подсписком `a`
    Other, // списки не равны и не являются подсписками друг друга
}

pub fn compare<T: Eq>(a: &[T], b: &[T]) -> Comparison {
    match a.len().cmp(&b.len()) {
        Ordering::Less => if is_sublist_of(a, b) { Comparison::Sublist } else { Comparison::Other }
        Ordering::Equal => if a == b { Comparison::Equal } else { Comparison::Other }
        Ordering::Greater => if is_sublist_of(b, a) { Comparison::Superlist } else { Comparison::Other }
    }
}

fn is_sublist_of<T: Eq>(a: &[T], b: &[T]) -> bool {
    b.windows(a.len()).any(|w| w == a)
}

#[cfg(test)]
mod tests {
    use crate::{compare, Comparison::*};

    #[test]
    fn a_sublist_b() {
        assert_eq!(compare(&[1, 2, 3], &[1, 2, 3, 4, 5]), Sublist);
        assert_eq!(compare(&[3, 4, 5], &[1, 2, 3, 4, 5]), Sublist);
        assert_eq!(compare(&[2, 3, 4], &[1, 2, 3, 4, 5]), Sublist);
    }

    #[test]
    fn a_subsequence_is_not_sublist() {
        assert_eq!(compare(&[1, 3, 5], &[1, 2, 3, 4, 5]), Other);
    }

    #[test]
    fn a_equals_b() {
        assert_eq!(compare::<i32>(&[], &[]), Equal);
        assert_eq!(compare(&[1, 3, 2], &[1, 3, 2]), Equal);
    }

    #[test]
    fn a_superlist_b() {
        assert_eq!(compare(&[1, 3, 2], &[3, 2]), Superlist);
    }
}