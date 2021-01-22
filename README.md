# web-grep

## What this?

Grep for HTML or XML.

```bash
$ echo '<a>Hello</a>' | web-grep '<a>{}</a>'
Hello
```

```bash
$ echo '<a>Hello</a>' | web-grep '<a>{html}</a>' --json
{"html":"Hello"}
```

```bash
# List up all <p>-innerHTML
$ cat << EOM | web-grep '<p>{}</p>'
<body>
  <p>hello</p>
  <div>
    <p>world</p>
  </div>
</body>
EOM
hello
world
```

```bash
# filtering with attributes
$ cat << EOM | web-grep '<p class=here>{}</p>'
<body>
  <p class="not-here">hello</p>
  <div>
    <p class="here">world</p>
  </div>
</body>
EOM
world
```

```bash
# Place-holder {} can be attribute
$ cat << EOM | web-grep '<p class={}>world</p>'
<body>
  <p class="not-here">hello</p>
  <div>
    <p class="here">world</p>
  </div>
</body>
EOM
here
```

## How this?

This is just a CLI for an awesome library, [tanakh/easy-scraper](https://github.com/tanakh/easy-scraper).

## Installation

1. Install cargo
    - Recommended Way: Install [rustup](https://rustup.rs/)
1. Then,
    - `cargo install web-grep`

## Usage

```bash
$ web-grep <QUERY> [INPUT]
```

The `QUERY` is a HTML (XML) Pattern.

Patterns are valid HTML structures which has placeholders for innerHTMLs or attributes.
`web-grep` has various placeholders for cases.

## Placeholders

### Anonymous Palceholder `{}`

If you need exact one placeholer in the pattern, use `{}`.

```html
<p>{}</p>
```

```html
<p class="here">
    <q>{}</q>
</p>
```

`web-grep` outputs all texts matching for `{}`.

```bash
$ echo "<p>1</p><p>2</p><p>3</p>" | web-grep "<p>{}</p>"
1
2
3
```

### Numbered Placeholders `{n}`

```html
<a href="{1}">{2}</a>
```

`web-grep` outputs matched texts for `{1}`, `{2}`... in order, separated by `\t`.

```bash
$ echo '<a href=hoge>fuga</a>' | web-grep "<a href={2}>{1}</a>"
fuga	hoge
```

The delimiter can be specified with `-F`.

```bash
$ echo '<a href=hoge>fuga</a>' | web-grep "<a href={2}>{1}</a>" -F ' '
fuga hoge
```

### Named Placeholders `{xxx}`

```html
<a href="{href}">{innerHTML}</a>
```

The output can be formatted as JSON with `--json`.

```bash
$ echo '<a href=hoge>fuga</a>' | web-grep "<a href={href}>{html}</a>" --json
{"href":"hoge","html":"fuga"}
```
