# quicklines

A simple CLI tool for efficient sampling of lines from large files.

Usage:

```bash
quicklines -c HOW_MANY_LINES my-huge-file.txt
```

`quicklines` will return _at most_ the requested number of lines from uniformly spaced parts of the input file.

The program simply cuts the input file into uniformly spaced chunks and prints the first sentence in each chunk.

If the requested number of lines `C` is orders of magnitude smaller than the total number of lines `N`, and if line lengths are
relatively uniform in the input, you should get back exactly the requested number.

You may get _less_ than `C` if the program encounters a "conflict" where the first line of a chunk is only found in a
subsequent chunk. This happens more often the closer `C` is to `N`.

The implementation relies on `mmap` to work efficiently.


