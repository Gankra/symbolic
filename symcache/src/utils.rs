/// A quick binary search by key.
pub fn binsearch_by_key<'a, T, B, F>(slice: &'a [T], item: B, mut f: F) -> Option<&'a T>
    where B: Ord, F: FnMut(&T) -> B
{
    let mut low = 0;
    let mut high = slice.len();

    while low < high {
        let mid = (low + high) / 2;
        let cur_item = &slice[mid as usize];
        if item < f(cur_item) {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    if low > 0 && low <= slice.len() {
        Some(&slice[low - 1])
    } else {
        None
    }
}

#[test]
fn test_binsearch() {
    let seq = [0u32, 2, 4, 6, 8, 10];
    let m = binsearch_by_key(&seq[..], 5, |&x| x);
    assert_eq!(*m.unwrap(), 4);
}