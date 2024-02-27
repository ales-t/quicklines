# quicklines

A simple CLI tool for efficient sampling of lines from large files.

Lines are selected randomly and by default, the output may contain the same line multiple times
(i.e. sampling with replacement).

Usage:

```bash
quicklines -c HOW_MANY_LINES my-huge-file.txt
```

`quicklines` will return the requested number of lines from random positions in the input file.

If `--deduplicate` is set, the returned number may be smaller (duplicate samples will be discarded).

The implementation relies on `mmap` to work efficiently.