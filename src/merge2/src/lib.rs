use std::collections::HashMap;
use std::ops::Range;

//TODO is it possible(and can it make sense) to generalize usize?
//TODO is it possible to generalize Vec in any iterable?
//TODO is it possible to generalize Range to make it also work with RangeInclusive?
//     are there other types of ranges?
pub fn merge(intervals: Vec<Range<usize>>) -> Vec<Range<usize>> {
    let mut ret: Vec<Range<usize>> = vec![];
    if intervals.len() == 0 {
        return ret;
    }

    let mut borders: HashMap<usize, i32> = HashMap::with_capacity(intervals.len() * 2);

    for i in intervals.iter() {
        *borders.entry(i.start).or_insert(0) += 1;
        *borders.entry(i.end).or_insert(0) -= 1
    }

    let mut v: Vec<_> = borders.iter().collect();
    v.sort();

    let mut level = 0;
    let mut start = 0;
    for (idx, val) in v {
        if level == 0 {
            start = *idx;
        }
        level += val;
        if level == 0 {
            ret.push(start..(*idx))
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merge2() {
        // TODO accept inclusive ranges, like 3..=6
        assert_eq!(merge(vec![]), []);
        assert_eq!(merge(vec![1..3, 4..6]), [1..3, 4..6]);
        assert_eq!(merge(vec![1..3, 4..6, 5..20]), [1..3, 4..20]);
        assert_eq!(merge(vec![1..3]), [1..3]);
        assert_eq!(merge(vec![3..6, 1..3]), [1..6]);
        assert_eq!(merge(vec![3..6, 1..3, 6..7]), [1..7]);
        assert_eq!(merge(vec![1..6, 1..3]), [1..6]);
        assert_eq!(merge(vec![1..3, 1..6]), [1..6]);
        assert_eq!(merge(vec![1..16, 1..3, 5..16, 20..21]), [1..16, 20..21]);
        assert_eq!(merge(vec![3..6, 1..3, 7..8]), [1..6, 7..8]);
        assert_eq!(merge(vec![1..16, 1..3, 5..16]), [1..16]);
        assert_eq!(merge(vec![1..16, 1..3, 5..16, 20..21]), [1..16, 20..21]);
    }
}
