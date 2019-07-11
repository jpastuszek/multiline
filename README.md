# Example usage

## Multiline

### Multiple interlaced streams of multiline log data

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

### Joining stack traces

Match every line that starts with space as belonging to previous message with `negate`.

```sh
multiline --message-pattern '^\s' --negate --join " " <<EOF
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
Exception in thread "main" java.lang.NullPointerException         at com.example.myproject.Book.getTitle(Book.java:16)         at com.example.myproject.Author.getBookTitles(Author.java:25)         at com.example.myproject.Bootstrap.main(Bootstrap.java:14)
Exception in thread "main" java.lang.NullPointerException         at com.example.myproject.Book.getTitle(Book.java:16)         at com.example.myproject.Author.getBookTitles(Author.java:25)         at com.example.myproject.Bootstrap.main(Bootstrap.java:14)
Hello world!
Exception in thread "main" java.lang.NullPointerException         at com.example.myproject.Book.getTitle(Book.java:16)         at com.example.myproject.Author.getBookTitles(Author.java:25)         at com.example.myproject.Bootstrap.main(Bootstrap.java:14)
```

### Joining lines ending with special escape

Use `match-last` with `negate` to anchor on the lines without the escape.
Add `strip-pattern` to ensure that the escape is not part of the output.

```sh
multiline --message-pattern ' \\$' --negate --match-last --join " " --strip-pattern <<EOF
Exception in thread "main" java.lang.NullPointerException \
        at com.example.myproject.Book.getTitle(Book.java:16) \
        at com.example.myproject.Author.getBookTitles(Author.java:25) \
        at com.example.myproject.Bootstrap.main(Bootstrap.java:14)
Exception in thread "main" java.lang.NullPointerException \
        at com.example.myproject.Book.getTitle(Book.java:16) \
        at com.example.myproject.Author.getBookTitles(Author.java:25) \
        at com.example.myproject.Bootstrap.main(Bootstrap.java:14)
Hello world!
Exception in thread "main" java.lang.NullPointerException \
        at com.example.myproject.Book.getTitle(Book.java:16) \
        at com.example.myproject.Author.getBookTitles(Author.java:25) \
        at com.example.myproject.Bootstrap.main(Bootstrap.java:14)
EOF
```

Produces:
```
Exception in thread "main" java.lang.NullPointerException         at com.example.myproject.Book.getTitle(Book.java:16)         at com.example.myproject.Author.getBookTitles(Author.java:25)         at com.example.myproject.Bootstrap.main(Bootstrap.java:14)
Exception in thread "main" java.lang.NullPointerException         at com.example.myproject.Book.getTitle(Book.java:16)         at com.example.myproject.Author.getBookTitles(Author.java:25)         at com.example.myproject.Bootstrap.main(Bootstrap.java:14)
Hello world!
Exception in thread "main" java.lang.NullPointerException         at com.example.myproject.Book.getTitle(Book.java:16)         at com.example.myproject.Author.getBookTitles(Author.java:25)         at com.example.myproject.Bootstrap.main(Bootstrap.java:14)
```