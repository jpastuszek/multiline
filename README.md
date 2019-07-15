[![Latest Version]][crates.io] [![Documentation]][docs.rs] ![License]

# Multiline

`multiline` is a CLI utility that can be used to transform realtime log message streams (e.g. log entries from application log file) that contain messages spanning multiple lines (e.g. stack traces) into streams with one message per line.

It can be useful as pre-processing step before sending log file data to syslog which would otherwise interpret messages spanning over multiple lines as separate messages one per line.

# Usage

`multiline` program will read UTF-8 data from from standard input, one line at a time and write messages spawning potentially multiple lines joined by `--join` string to standard output.

To configure message beginning or end matching algorithm `--message-pattern` regex, `--negate` and `--match-last` flags can be used. `--strip-pattern` flag can by used to remove matched message pattern from output.

Additionally `multiline` can handle multiple streams of log data interlaced together in standard input with use of `--stream-id-pattern` regex to match each stream identifying pattern.

Real time processing parameters can be configured with `--max-size` flag which limits maximum number of lines that each stream will accumulate and `--max-duration` which limits time duration that each message will continue to aggregate lines for. After one of this limits is reached concatenated message is flushed to the output.

Specifying `--timestamp` flag will enable printing of timestamps (in ISO 8601 UTC format) for each message constructed based on time when first line of the message was read.

## Examples

### Joining stack traces

Match every line that starts with space as belonging to previous message with `negate`.

```sh
multiline --message-pattern '^\s+' --negate --strip-pattern --join "; " <<EOF
Exception in thread "main" java.lang.NullPointerException
        at com.example.myproject.Book.getTitle(Book.java:16)
        at com.example.myproject.Author.getBookTitles(Author.java:25)
        at com.example.myproject.Bootstrap.main(Bootstrap.java:14)
Exception in thread "main" java.lang.NullPointerException
        at com.example.myproject.Book.getTitle(Book.java:16)
        at com.example.myproject.Author.getBookTitles(Author.java:25)
        at com.example.myproject.Bootstrap.main(Bootstrap.java:14)
Hello world!
Exception in thread "main" java.lang.NullPointerException
        at com.example.myproject.Book.getTitle(Book.java:16)
        at com.example.myproject.Author.getBookTitles(Author.java:25)
        at com.example.myproject.Bootstrap.main(Bootstrap.java:14)
EOF
```

Produces:
```
Exception in thread "main" java.lang.NullPointerException; at com.example.myproject.Book.getTitle(Book.java:16); at com.example.myproject.Author.getBookTitles(Author.java:25); at com.example.myproject.Bootstrap.main(Bootstrap.java:14)
Exception in thread "main" java.lang.NullPointerException; at com.example.myproject.Book.getTitle(Book.java:16); at com.example.myproject.Author.getBookTitles(Author.java:25); at com.example.myproject.Bootstrap.main(Bootstrap.java:14)
Hello world!
Exception in thread "main" java.lang.NullPointerException; at com.example.myproject.Book.getTitle(Book.java:16); at com.example.myproject.Author.getBookTitles(Author.java:25); at com.example.myproject.Bootstrap.main(Bootstrap.java:14)
```

### Joining lines ending with special escape

Use `match-last` with `negate` to anchor on the lines without the escape.
Add `strip-pattern` to ensure that the escape is not part of the output.

```sh
multiline --message-pattern ' >$' --negate --match-last --strip-pattern --join "; " <<EOF
Exception in thread "main" java.lang.NullPointerException >
at com.example.myproject.Book.getTitle(Book.java:16) >
at com.example.myproject.Author.getBookTitles(Author.java:25) >
at com.example.myproject.Bootstrap.main(Bootstrap.java:14)
Exception in thread "main" java.lang.NullPointerException >
at com.example.myproject.Book.getTitle(Book.java:16) >
at com.example.myproject.Author.getBookTitles(Author.java:25) >
at com.example.myproject.Bootstrap.main(Bootstrap.java:14)
Hello world!
Exception in thread "main" java.lang.NullPointerException >
at com.example.myproject.Book.getTitle(Book.java:16) >
at com.example.myproject.Author.getBookTitles(Author.java:25) >
at com.example.myproject.Bootstrap.main(Bootstrap.java:14)
EOF
```

Produces:
```
Exception in thread "main" java.lang.NullPointerException; at com.example.myproject.Book.getTitle(Book.java:16); at com.example.myproject.Author.getBookTitles(Author.java:25); at com.example.myproject.Bootstrap.main(Bootstrap.java:14)
Exception in thread "main" java.lang.NullPointerException; at com.example.myproject.Book.getTitle(Book.java:16); at com.example.myproject.Author.getBookTitles(Author.java:25); at com.example.myproject.Bootstrap.main(Bootstrap.java:14)
Hello world!
Exception in thread "main" java.lang.NullPointerException; at com.example.myproject.Book.getTitle(Book.java:16); at com.example.myproject.Author.getBookTitles(Author.java:25); at com.example.myproject.Bootstrap.main(Bootstrap.java:14)
```

### Join messages from multiple interlaced streams of log data

The `stream-id-pattern` regex will match the stream name. For each stream a separate concatenation buffer will be maintained.
Timestamp is used to match the beginning of the message in each stream.

```sh
multiline --message-pattern '^20' --stream-id-pattern '.*: ' --join " " <<EOF
bar: 2019-07-05 15:42:16 Lorem ipsum dolor sit amet,
foo: 2019-07-05 15:42:16 Lorem ipsum dolor sit amet,
foo: consectetur adipiscing elit
bar: consectetur adipiscing elit
foo: 2019-07-05 15:42:17 Phasellus eleifend scelerisque lorem,
foo: a placerat ex dictum iaculis.
foo: 2019-07-05 15:42:18 Nam porta hendrerit fermentum.
foo: 2019-07-05 15:42:19 Vivamus vitae faucibus purus.
bar: 2019-07-05 15:42:17 Phasellus eleifend scelerisque lorem,
foo: 2019-07-05 15:42:20 Cras suscipit lacus ex.
bar: a placerat ex dictum iaculis.
bar: 2019-07-05 15:42:18 Nam porta hendrerit fermentum.
bar: 2019-07-05 15:42:19 Vivamus vitae faucibus purus.
foo: 2019-07-05 15:42:21 Phasellus sed nulla faucibus,
foo: bibendum dolor vitae,
foo: faucibus dui.
bar: 2019-07-05 15:42:20 Cras suscipit lacus ex.
bar: 2019-07-05 15:42:21 Phasellus sed nulla faucibus,
bar: bibendum dolor vitae,
bar: faucibus dui.
EOF
```

Produces:
```
foo: 2019-07-05 15:42:16 Lorem ipsum dolor sit amet, consectetur adipiscing elit
foo: 2019-07-05 15:42:17 Phasellus eleifend scelerisque lorem, a placerat ex dictum iaculis.
foo: 2019-07-05 15:42:18 Nam porta hendrerit fermentum.
bar: 2019-07-05 15:42:16 Lorem ipsum dolor sit amet, consectetur adipiscing elit
foo: 2019-07-05 15:42:19 Vivamus vitae faucibus purus.
bar: 2019-07-05 15:42:17 Phasellus eleifend scelerisque lorem, a placerat ex dictum iaculis.
bar: 2019-07-05 15:42:18 Nam porta hendrerit fermentum.
foo: 2019-07-05 15:42:20 Cras suscipit lacus ex.
bar: 2019-07-05 15:42:19 Vivamus vitae faucibus purus.
bar: 2019-07-05 15:42:20 Cras suscipit lacus ex.
foo: 2019-07-05 15:42:21 Phasellus sed nulla faucibus, bibendum dolor vitae, faucibus dui.
bar: 2019-07-05 15:42:21 Phasellus sed nulla faucibus, bibendum dolor vitae, faucibus dui.
```

### Timestamp messages

Enable timestamps with `--timestamp` flag.
```sh
seq 1 10 | while read I; do echo $I; sleep 0.2; done | multiline --message-pattern '.' --timestamp
```

Produces:
```
2019-07-12T13:29:12.872375Z 1
2019-07-12T13:29:13.071095Z 2
2019-07-12T13:29:13.273488Z 3
2019-07-12T13:29:13.475585Z 4
2019-07-12T13:29:13.677506Z 5
2019-07-12T13:29:13.879438Z 6
2019-07-12T13:29:14.082132Z 7
2019-07-12T13:29:14.284793Z 8
2019-07-12T13:29:14.487547Z 9
2019-07-12T13:29:14.690270Z 10
```

[crates.io]: https://crates.io/crates/multiline
[Latest Version]: https://img.shields.io/crates/v/multiline.svg
[Documentation]: https://docs.rs/multiline/badge.svg
[docs.rs]: https://docs.rs/multiline
[License]: https://img.shields.io/crates/l/multiline.svg
