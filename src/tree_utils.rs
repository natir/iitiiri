//! Function usefull in tree exploration

/* std use */

/* crate use */

/* project use */

#[inline(always)]
pub fn index2level(index: usize) -> usize {
    (!(index as u64)).trailing_zeros() as usize
}

#[inline(always)]
pub fn right(index: usize) -> Option<usize> {
    let level = index2level(index);
    if level == 0 {
        None
    } else {
        Some(right_uncheck(index, level))
    }
}

#[inline(always)]
pub fn right_uncheck(index: usize, level: usize) -> usize {
    index + (1 << (level - 1))
}

#[inline(always)]
pub fn left(index: usize) -> Option<usize> {
    let level = index2level(index);
    if level == 0 {
        None
    } else {
        Some(left_uncheck(index, level))
    }
}

#[inline(always)]
pub fn left_uncheck(index: usize, level: usize) -> usize {
    index - (1 << (level - 1))
}

#[inline(always)]
pub fn parent(index: usize) -> usize {
    let level = index2level(index);

    let ofs = 1 << level;
    if (index >> (level + 1)) & 1 == 1 {
        index - ofs
    } else {
        index + ofs
    }
}

#[inline(always)]
pub fn rightmost_leaf(index: usize) -> usize {
    let level = index2level(index);
    index + ((1 << level) - 1)
}

#[inline(always)]
pub fn leftmost_leaf(index: usize) -> usize {
    let level = index2level(index);
    index - ((1 << level) - 1)
}

#[inline(always)]
pub fn index2index_in_level(index: usize) -> usize {
    ((index + 1) / (1 << index2level(index)) - 1) / 2
}

#[inline(always)]
pub fn index_in_level2index(level: usize, index: usize) -> usize {
    (1 << level) * (2 * index + 1) - 1
}

#[cfg(test)]
mod tests {
    /* std use */

    /* crate use */

    /* project use */
    use super::*;

    #[test]
    fn tree_traversal() {
        // Node index2level
        assert_eq!(
            (0..10).map(index2level).collect::<Vec<usize>>(),
            vec![0, 1, 0, 2, 0, 1, 0, 3, 0, 1]
        );

        // right
        assert_eq!(
            (0..10).map(right).collect::<Vec<Option<usize>>>(),
            vec![
                None,
                Some(2),
                None,
                Some(5),
                None,
                Some(6),
                None,
                Some(11),
                None,
                Some(10)
            ]
        );

        // left
        assert_eq!(
            (0..10).map(left).collect::<Vec<Option<usize>>>(),
            vec![
                None,
                Some(0),
                None,
                Some(1),
                None,
                Some(4),
                None,
                Some(3),
                None,
                Some(8)
            ]
        );

        // parent
        assert_eq!(
            (0..10).map(parent).collect::<Vec<usize>>(),
            vec![1, 3, 1, 7, 5, 3, 5, 15, 9, 11]
        )
    }

    #[test]
    fn leaf_leftmost() {
        assert_eq!(leftmost_leaf(7), 0);
        assert_eq!(leftmost_leaf(5), 4);
        assert_eq!(leftmost_leaf(11), 8);
    }
}
