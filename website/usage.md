# Usage

Create node collections:
```rust
struct Annotations {
...
}

let mut nodes = Vec::new();
for ...something... {
	let start: usize = ...
	let stop: usize = ...
	let annotations: Annotations = ...

	nodes.push(iitiiri::Node::new(start, stop, annotations));
}
```

Build interval tree:
```rust
let interval_tree: iitiiri::Iit<usize, Annotations> = iitiiri::Iit::new(nodes);
```

Build interval tree with interpolation index:
```rust
const NUMBER_OF_DOMAIN: usize = 32;
let interval_tree: iitiiri::Iitii<usize, Annotations, NUMBER_OF_DOMAIN> = iitiiri::Iitii::new(nodes);
```

Query interval tree (work for Iit or Iitii):
```rust
let result: Vec<&Annotations> = interval_tree.query(start, stop)
```

## Implicit Interval Tree


During the building step, we just sort node by start of interval to build an implicit binary tree, and compute for each node the maximal end of interval in child.

Querying just involves going down the binary tree to find all the intervals that share an overlap with the query.

You could found many details in [Bedtk publication](https://doi.org/10.1093/bioinformatics/btaa827)


## Implicit Interval Tree with Interpolation Index

Is same as Iit, but durring building step we split interval in `NUMBER_OF_DOMAIN`, for each domain found an affine function that feet on begin of interval.

Durring quering step we use affine function to guess a node lower than root in tree and save some times.

`NUMBER_OF_DOMAIN` should be between 1 and infinity, if `NUMBER_OF_DOMAIN` is large the build time and memory usage is little bit larger, but query time is shorter, you should make a choice.

## Resume

| | build time | query time | memory usage |
|-|-|-|-|
| `Iit` | O(n log(n)) | O(n log(n)) | n |
| `Iitii` | O(n log(n) + C)  | O(n log(n) - C) | n + NUMBER_OF_DOMAIN |

n are number of node, and C the effect of guessing subtree match node
