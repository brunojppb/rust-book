# Minigrep

search for lines of text in any given text file.

## How-to:

Run the following:

```shell
cargo run -- "Text to search here" poem.txt
```

You should get back the text lines that contain your query.

### case-insensitive search

To perform case-insensitive search, use the environment variable `IGNORE_CASE`:

```shell
IGNORE_CASE=1 cargo run -- "to" poem.txt
```
