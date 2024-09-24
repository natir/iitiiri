---
title: 'Iitiiri: '
tags:
  - Rust
  - IntervalTree
  - Annotation
authors:
  - name: Pierre Marijon
    orcid: 0000-0002-6694-6873
    affiliation: 1
affiliations:
 - name: Laboratoire de Biologie Médicale Multisites SeqOIA, FMG2025, Paris, France
   index: 1
date: 28 August 2024
bibliography: paper.bib
---

# Summary

Intersection searching between sets of intervals is a useful task for many bioinformatics problems, such as finding the regions of interest affected by a variant, or even calculating the coverage of the same type of region.

Several tools already exist to address these issues as bedtools, bedtk and iitii. Iitiiri reuse idea from bedtk and iitii and implements them in the Rust language to take advantage of its parallelization capabilities, to improve performances in building time.

# Statement of need

To know the impact of a variantion in the genome of an individual, it is necessary to determine which biological region it will affect. Biological regions and variant can be represented as intervals in $\mathbb{N}$. With these representations the problem of associate a variant to biological region affected could be see as a trouble to found intersection between an interval and a set of interval.

Bedtk[@bedtk] is a C library that adress this question by implement an implicit binary search tree, iitii[@iitii] reuse same idea but introduce an interpolation index to start binary search from a lower level in tree. But this implementation aren't multi-thread, iitiiri by implement this method in Rust use the capability of paralellisation of these language[@why_rust].

# Method

Bedtk method are based on binary search tree (BST), a BST can be construct by sort array of interval, each node of tree is an element on array and index of element in array can use to infere tree topology.

# Result

# Acknowledgements

We acknowledge Michael F. Lin, for the quality of the description of its algorithm and its ideas, from which I drew a great deal of inspiration.

# References
