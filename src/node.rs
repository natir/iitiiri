//! Declaration of node

/* std use */

/* crate use */

/* project use */

#[derive(Clone)]
pub struct Node<P, O> {
    start: P,
    stop: P,
    object: O,
    max_end: P,
}

impl<P, O> std::fmt::Debug for Node<P, O>
where
    P: std::fmt::Debug,
    O: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Node")
            .field("start", &self.start)
            .field("stop", &self.stop)
            .field("object", &self.object)
            .field("max_end", &self.max_end)
            .finish()
    }
}

impl<P, _O> std::cmp::PartialEq for Node<P, _O>
where
    P: std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.stop == other.stop
    }
}

impl<P, _O> std::cmp::Eq for Node<P, _O> where P: std::cmp::PartialEq {}

impl<P, _O> std::cmp::PartialOrd for Node<P, _O>
where
    P: std::cmp::PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.start.partial_cmp(&other.start) {
            Some(std::cmp::Ordering::Equal) => self.stop.partial_cmp(&other.stop),
            other => other,
        }
    }
}

impl<P, _O> std::cmp::Ord for Node<P, _O>
where
    P: std::cmp::Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.start.cmp(&other.start) {
            std::cmp::Ordering::Equal => self.stop.cmp(&other.stop),
            other => other,
        }
    }
}

impl<P, O> Node<P, O>
where
    P: std::marker::Copy,
{
    pub fn new(start: P, stop: P, object: O) -> Self {
        Self {
            max_end: stop,
            start,
            stop,
            object,
        }
    }

    #[cfg(test)]
    pub fn new_full(start: P, stop: P, object: O, max_end: P) -> Self {
        Self {
            start,
            stop,
            object,
            max_end,
        }
    }

    pub fn start(&self) -> &P {
        &self.start
    }

    pub fn stop(&self) -> &P {
        &self.stop
    }

    pub fn object(&self) -> &O {
        &self.object
    }

    pub fn max_end(&self) -> &P {
        &self.max_end
    }

    pub fn set_max_end(&mut self, value: P) {
        self.max_end = value;
    }
}

#[cfg(test)]
mod tests {
    /* std use */

    /* crate use */

    /* project use */
    use super::*;

    #[test]
    fn accessor() {
        let mut node = Node::<u64, f64>::new(1, 201, 1.0);

        assert_eq!(node.start(), &1);
        assert_eq!(node.stop(), &201);
        assert_eq!(node.object(), &1.0);
        assert_eq!(node.max_end(), &201);

        node.set_max_end(10);

        assert_eq!(node.max_end(), &10);
    }

    #[test]
    fn eq() {
        let n1 = Node::<u64, f64>::new(1, 201, 1.0);
        let mut n2 = Node::<u64, f64>::new(1, 201, 1.0);

        assert_eq!(n1, n2);

        n2.set_max_end(2);

        assert_eq!(n1, n2);

        let n2 = Node::<u64, f64>::new(1, 201, 2.0);

        assert_eq!(n1, n2);
    }

    #[test]
    fn order() {
        let n1 = Node::<u64, f64>::new(1, 201, 1.0);
        let n2 = Node::<u64, f64>::new(10, 201, 2.0);
        assert!(n1 < n2);

        let n2 = Node::<u64, f64>::new(1, 211, 2.0);
        assert!(n1 < n2);

        let n2 = Node::<u64, f64>::new(0, 201, 2.0);
        assert!(n1 > n2);

        let n2 = Node::<u64, f64>::new(1, 201, 2.0);
        assert_eq!(n1.cmp(&n2), std::cmp::Ordering::Equal);

        let n2 = Node::<u64, f64>::new(0, 201, 2.0);
        assert_eq!(n1.cmp(&n2), std::cmp::Ordering::Greater);

        let n2 = Node::<u64, f64>::new(2, 201, 2.0);
        assert_eq!(n1.cmp(&n2), std::cmp::Ordering::Less);
    }

    #[test]
    fn debug() {
        let n = Node::<u64, f64>::new(1, 201, 1.0);

        assert_eq!(
            format!("{:?}", n),
            "Node { start: 1, stop: 201, object: 1.0, max_end: 201 }".to_string()
        );
    }
}
