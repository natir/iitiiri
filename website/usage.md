# Usage

Build interval tree:
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

let iit: iitiiri::Iit<usize, Annotations> = iitiiri::Iit::new(nodes);
```

Query interval tree:
```rust
let result: Vec<&Annotations> = iit.query(start, stop)
```
