# web-grep

## What this

```bash
$ echo "<html><body><p>hoge</p><p><p>fuga</p><p>xxx</p></p></body></html>" | web-grep "<p>{}</p>"
hoge
fuga
xxx
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
