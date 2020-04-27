# web-grep

## What this

```bash
# List up all <p>-innerHTML
$ echo "<html><body><p>hoge</p><p><p>fuga</p><p>xxx</p></p></body></html>" | web-grep "<p>{}</p>"
hoge
fuga
xxx

# List up `href` of <a> with class `box` and which contains `<span>date</span>`
$ echo '<div><a class="box" href="url1"><span>date</span></a><a class="box" href="url2"></a> | web-grep '<a class=box href={}><span>date</span></a>'
url1
```

## How this

This is just a CLI for [tanakh/easy-scraper](https://github.com/tanakh/easy-scraper).

## Usage

```
$ web-grep <query>
```

The `query` is HTML pattern.
Patterns like 

```html
<p>{}</p>
```

```html
<p class="here">
    <q>{}</q>
</p>
```

`web-grep` outputs all text for `{}`.

(Note: the original library `easy-scraper` use `{{xxx}}` for placeholders, but this CLI use only `{}`)

## Build

Get `cargo` then,

```bash
cargo build --release

cp target/release/web-grep /your/local/path/bin/
```

or use `make install`.
