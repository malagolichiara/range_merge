use std::ops::Range;

//TODO is it possible(and can it make sense) to generalize usize?
//TODO is it possible to generalize Vec in any iterable?
//TODO is it possible to generalize Range to make it also work with RangeInclusive?
//     are there other types of ranges?
pub fn merge(intervals: Vec<Range<usize>>) -> Vec<Range<usize>> {
    let mut ret: Vec<Range<usize>> = vec![];
    let mut tmp: Vec<&Range<usize>> = intervals.iter().collect();

    if intervals.len() == 0 {
        return ret;
    }

    tmp.sort_by(|a, b| a.start.cmp(&b.start));

    let mut start = tmp[0].start;
    let mut end = tmp[0].end;
    for (ptr, b) in tmp.iter().enumerate() {
        if b.end <= b.start {
            continue;
        }
        if end < b.start {
            ret.push(start..end);
            start = tmp[ptr].start;
            end = tmp[ptr].end;
        // } else if end >= b.start && b.end > end {
        } else if b.end > end {
            end = b.end;
        }
    }
    ret.push(start..end);

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merge1() {
        // TODO accept inclusive ranges, like 3..=6
        assert_eq!(merge(vec![]), []);
        assert_eq!(merge(vec![1..3, 4..6]), [1..3, 4..6]);
        assert_eq!(merge(vec![1..3, 4..6, 5..20]), [1..3, 4..20]);
        assert_eq!(merge(vec![1..3]), [1..3]);
        assert_eq!(merge(vec![1..3, 7..2]), [1..3]);
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
