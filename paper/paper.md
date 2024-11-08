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
- root of tree is at index $2^K - 1$
- level of a node: trailing one in binary representation of index
- left child node index: $index - 2^{level-1}$
- right child node index: $index + 2^{level-1}$
- parent node index: $index \pm 2^{level}$

![A binary search tree build from array of interval. Nodes are store in array tree representation are build from index of node, the most right node label as imaginary are just present in tree structure and not alocate in array. Node struct store range (in upper part) and the `max_end` value (in lower part).]{label="fig_bst"}(figure/bst.png)

Node of tree store interval information and `max_end` value that correspond to maximal end of interval in subtree of this nodes. Figure \ref{fig_bst} show an example of BST with node tree struct and corresponding array.

## Tree quering \label{sec_tree_quering}



## Interpolate Index building

Interpolate index reuse same structure as bst present earlier, with some adition.
Array are divide in $N$ domain, for each domain and each level clairiere compute an affine function that fit start of each nodes of domain at a specific level the affine function with the lowest error rate for all nodes of specific domain are kept.

This affine function determines a sub-tree closer to the query than the root of the tree. Even if the affine function minimizes the error, the index estimate may choose a sub-tree that doesn't include all the intervals that overlap with the query. We therefore need to add an equivalent to the `max_end` value, the `outside_max_end`, to determine whether the next sub-tree could share an overlap with the query.

## Interpolate Index query

For interpolate index quering we use exactly same algorithm as describe in section \ref{sec_tree_quering} but applying it to the sub-tree whose node is chosen by the affine function of the query domain.

# Result

To evaluate performance of my implementation I use variant produce by [@hg00_variant], clinvar[@clinvar] release 30/07/2024 and variant of chromosome 2 of gnomad exon version 2.1.1 [@gnomad2.1] and Ensembl annotation of GRCh38.92.

I compare my Clairiere Tree (*clairiere*), to bedtk tree struct (*cgranges*), to rust-bio[@rustbio] bedtk tree struct reimplementation (*rust-bio*) and compare my Implicite Interval Tree Interpolate Index (*clairiere_interpolate*) to Michael F. Lin implementation (*iitii*).

A snakemake pipeline to reproduce experiment is available in project repository[^1].

## Run time and memory usage to build tree

## Run time in function of tree size

## Run time in function of query size

## Effect of affine interpolation

In previous section we see increasing number of domain haven't a clear impact on run time. To check behavior of this algorithm we add a conditional compilation to get at which level estimator guess and if the guess algorithm must perform a correction and how .

![How number of domain impact estimator metrics.](figure/effect_affine_interpolation){label="effect_affine_interpolation"}

As we can see in figure \ref{effect_affine_interpolation} with more domain prediction level is more near to leaf and correction level seems to same.

# Acknowledgements

I would like to thank my brother for the beautiful name he found for this crate.
I acknowledge Michael F. Lin, for the quality of the description of its algorithm and its ideas.
I'd also like to dedicate this work to my aunt who passed away while I was writing this paper.

# References

[^1]: Check paper/experiment/Readme.md file to get instruction on how to reproduce experiment
