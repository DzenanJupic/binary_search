use std::cmp::Ordering;

pub fn binary_search<T: Ord>(slice: &[T], element: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = slice.len();

    while left <= right {
        let i = (right + left) / 2;
        let middle = unsafe { slice.get_unchecked(i) };

        match middle.cmp(element) {
            Ordering::Less => left = i + 1,
            Ordering::Equal => return Some(i),
            Ordering::Greater if i == 0 => return None,
            Ordering::Greater => right = i - 1
        }
    }

    None
}


#[test]
fn test() {
    let s: &[_] = &[1, 2, 3, 4, 5, 6, 7, 8];

    assert_eq!(binary_search(&s, &1), Some(0));
    assert_eq!(binary_search(&s, &4), Some(3));
    assert_eq!(binary_search(&s, &0), None);
    assert_eq!(binary_search(&s, &9), None);

    let s: &[_] = &[1, 2, 3, 4, 5, 6, 7, 8, 9];

    assert_eq!(binary_search(&s, &1), Some(0));
    assert_eq!(binary_search(&s, &4), Some(3));
    assert_eq!(binary_search(&s, &0), None);
    assert_eq!(binary_search(&s, &10), None);
}
