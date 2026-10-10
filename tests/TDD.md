# Test plan

TDD means test-driven development. Define the required result in a test.
Run the test before you change the application. Then change the application
until the test passes.

## Correct results

Run the normal tests:

```bash
cargo test --locked
```

`correctness.rs` checks the public library function. `cli_contract.rs` checks
the command-line tool. Some tests must fail with the current application.
Do not mark those tests as ignored. They define work that is not complete.

The required behavior is:

- Zero samples produce no output, including for an empty file.
- An empty file cannot provide a positive sample.
- One line is a valid input.
- A final line without a newline is a valid input line.
- Each output line ends with a newline. Add `\n` when the input line has none.
  This keeps repeated final lines separate. Keep other input bytes unchanged.
- A final newline does not create an extra empty input line.
- Every input line position has the same selection chance.
- Selection without repeats uses input positions, not distinct text.
- Impossible unique requests return an error before writing any lines.
- The same seed gives the same sample within the same implementation.
- Output errors reach the caller. The CLI also reports final buffer errors.

The chance tests use fixed seeds and broad count limits. They check repeated
selection, the first unique selection, and two-line sets selected without
repeats. They do not require one specific sequence of selected lines. The
tests do not prove equal chances for every possible input.

The CLI tests stop a command after five seconds. This protects the tests from
an endless loop. It is not a speed target.

## Speed limits

Run the speed tests separately:

```bash
cargo test --release --locked --test performance -- --ignored --test-threads=1 --nocapture
```

Use an otherwise idle machine. The tests create real files under
`/tmp/opencode`. They remove their own files after the test. They need about
256 MiB of free space when run with one test thread.

Each test measures a full scan that counts newline bytes. Each operation has
one warm-up run and three measured runs. The test uses the middle measured
time. File creation is not part of the measurement. File data is already in
memory. The output writer counts bytes but does not store them.

The starting limits are:

| Case | Input | Request | Limit |
| --- | --- | --- | --- |
| Small sample | 256 MiB; 128-byte lines | 32 lines, with repeats allowed | Half the scan time, plus 2 ms |
| Complete sample | 128 MiB; 128-byte lines | Every position, once each | 20 times the scan time, plus 100 ms |
| Long lines | 128 MiB; 256 KiB lines | 512 lines, with repeats allowed | 20 times the scan time, plus 100 ms |

These are broad limits, not claims that selection can match a line count.
Selection does more work. The complete-sample limit should expose the current
large cost of repeated attempts. The long-line limit should also expose a
future method that checks line starts but never switches to a scan.

Machine load can make a speed test fail. Repeat a failed speed test before you
change a limit. These tests do not measure disk reads when file data is absent
from memory. They do not measure real output-device speed.

## Method selection

The current public function does not report which method it uses. These tests
check cost instead of a method name. This lets the implementation change.

When the application has a separate method selector, add tests for its input
and result. Include small requests, large requests, long lines, slow lookup
progress, and a valid saved list of line starts. Do not add a public interface
only to make the current tests pass.

Keep the seed result repeatable when you add a method switch. A switch based
on measured run time can select a different method on the next run.

The existing `integration_test.rs` has some checks tied to exact seed results.
Review those checks when the selection method changes. Do not require a new,
correct method to reproduce the old biased sequence.
