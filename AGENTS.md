# Agent instructions

## Language

- Use only ASD-STE100 Simplified Technical English in responses, documentation,
  and comments.
- Use short sentences. Give one instruction in each sentence.
- Use the active voice. Use the same word for the same meaning.
- Define technical terms before you use them.
- Keep file paths, commands, code names, and required code syntax unchanged.
- Do not research ASD-STE100 for this task. Apply the rules that you know.

## Project purpose

`quicklines` is a Rust command-line tool. It selects lines from a large file.
It writes the selected lines to standard output, which is the normal output
of a command.

A sample is a group of selected lines. The `-c` option sets the sample size.
The default mode can select the same input line more than once.

The `--no-duplicates` option prevents a second selection of the same line
position. Two lines at different positions can have the same text.

A seed is a number that sets the random number sequence. The `--seed` option
sets this number. The same seed gives the same sample when the input, options,
and implementation stay the same.

## Data and selection

A byte is one unit of file data. A byte offset is a position measured in bytes
from the start of a file. Offset zero is the first position.

A newline is the byte `\n`. The current code uses this byte to find line ends.
The current code does not select a final line without a newline.

A memory map gives the code access to file data through memory addresses.
The operating system loads file data as necessary. The code uses `memmap2`
to make this map. A map does not require a separate copy of the full file.

The current code selects a random byte offset, not a random line number.
If the offset is at a line start, the code selects that line. Otherwise, the
code moves forward to the next line start. It copies the selected line,
including its newline, to the output.

Bias means that some lines have a larger chance of selection than other lines.
The current method has bias. A line after a long line has a larger chance of
selection. The first line can be selected only through offset zero.
Do not describe this method as equal-chance line selection.

## Files

- `src/lib.rs` contains the file map, line search, and selection code.
- `src/main.rs` reads command options and writes the output.
- `tests/integration_test.rs` tests the public library function.
- `tests/resources/` contains small test input files.
- `scripts/generate-fixtures.sh` makes large test input files with Bash and awk.
- `scripts/measure.py` checks results and measures run time.
- `scripts/benchmark.sh` runs file creation, the build, and the measurements.
- `BENCHMARKS.md` gives measured results, known errors, and proposed changes.

## Checks and changes

- Run `cargo test --locked` after a change to Rust code.
- Use `cargo build --release --locked` for speed measurements.
- Run `bash scripts/benchmark.sh` to repeat the full evaluation.
- Keep large generated files outside the repository. Use `/tmp/opencode`.
- Report whether a speed test uses file data already in memory.
- Note that the evaluation script returns status 1 when a result check fails.
- Check selection bias, line counts, repeated lines, and output errors.
- Keep user files and changes that are not part of the task.
- Do not change selection behavior without a request to do so.
