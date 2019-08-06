# frip

`frip` provides extremely basic functionality for calculating the fraction of reads in an experiment falling into peaks. 

Keep in mind that I whipped this up very quickly for a specific case, so I can't guarantee it isn't filled with bugs.

## usage

You only need to provide a bam file (cram and sam soon to come) and a bed file. The bam doesn't need to be indexed. The result is written to stdout.

```bash
frip a.bam a.bed > a.txt
```

## warning

Note that for now the bed file must be in full 6 column format - providing an invalid bed will silently produce a frip score of 0.0. This is something I've been meaning to fix, but for now something like this will do the trick:

```bash
awk '{$4=".";$5="0";$6="+"}1' original.bed | tr ' ' \\t > fixed.bed
```
