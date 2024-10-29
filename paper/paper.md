---
title: 'Clairiere a rust implementation of implicit interval tree with interpolation index.'
tags:
  - Rust
  - IntervalTree
  - Annotation
  - Variant
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

Intersection search between sets of intervals is a useful task for many bioinformatics problems, such as finding the regions of interest affected by a variant, or even calculating the coverage of the same type of region.

Several tools already exist to address these issues as bedtools, bedtk and iitii. Clairiere reuse idea from bedtk and iitii and implements them in the Rust language to take advantage of its parallelization capabilities, to improve performances in building time.

# Statement of need

To know the impact of a variation in the genome of an individual, it is necessary to determine which biological region it will affect. Biological regions and variant can be represented as intervals in $\mathbb{N}$. With these representations the problem of associate a variant to biological region affected could be see as a trouble to found intersection between an interval and a set of interval.

Bedtk[@bedtk] is a C library that adress this question by implement an implicit binary search tree, iitii[@iitii] reuse same idea but introduce an interpolation index to start binary search from a lower level in tree. But this implementation aren't multi-thread, Clairiere by implement this method in Rust use the capability of paralellisation of these language[@why_rust].

# Method

## Tree building

Bedtk method are based on binary search tree (BST), a BST can be construct by sort array of interval, each node of tree is an element on array and index of element in array can use to infere tree topology.

If we have an array of $2^{K+1} - 1$ element:

- tree have K + 1 levels
- level of a node, // TODO
- left child node index, $index - 2^{level-1}$
- right child node index, $index + 2^{level-1}$
- parent node
- root of tree is at index $2^K - 1$

![An example of binary search tree of ten intervals](figure/bst.png)

Node of tree store interval information and `max_end` value that correspond to maximal end of interval in subtree of this nodes.

## Tree quering

## Interpolate Index building

## Interpolate Index query

# Result

To evaluate performance of my implementation I use variant produce by [@hg00_variant], clinvar[@clinvar] release 30/07/2024 and variant of chromosome 2 of gnomad exon version 2.1.1 [@gnomad2.1] and Ensembl annotation of GRCh38.92.

I compare my Clairiere Tree (*clairiere*), to bedtk tree struct (*cgranges*) and rust-bio[@rustbio] bedtk tree struct reimplementation and compare my Implicite Interval Tree Interpolate Index (*clairiere_interpolate*) to Michael F. Lin implementation (*iitii*).

A snakemake pipeline to reproduce experiment is available in project repository[^1].

## Run time and memory usage to build tree

## Run time in function of tree size

## Run time in function of query size

## Effect of affine interpolation

# Acknowledgements

I acknowledge Michael F. Lin, for the quality of the description of its algorithm and its ideas.
I'd also like to dedicate this work to my aunt who passed away while I was writing this paper.

# References

[^1]: Check paper/experiment/Readme.md file to get instruction on how to reproduce experiment
