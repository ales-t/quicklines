# quicklines

A simple CLI tool for efficient sampling of lines from large files.

Lines are selected randomly and by default, the output may contain the same line multiple times
(i.e. sampling with replacement).

Usage:

```bash
quicklines -c HOW_MANY_LINES my-huge-file.txt
```

`quicklines` will return the requested number of lines from random positions in the input file.

Optionally, you can sample without replacement by using `--no-duplicates`. 
Be careful with this option, if you ask for a sample which is too large, this may cause the program
to run forever.

The implementation relies on `mmap` to work efficiently.