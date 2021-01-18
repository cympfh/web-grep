# web-grep

## What this?

Grep for HTML or XML.

```bash
$  echo '<a>Hello</a>' | web-grep '<a>{}</a>'
Hello

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

This is a just CLI for an awesome library, [tanakh/easy-scraper](https://github.com/tanakh/easy-scraper).

## Installation

1. Install cargo
    - Recommended Way: Install [rustup](https://rustup.rs/)
1. Then,
    - `cargo install web-grep`

## Usage

```bash
$ web-grep <query>
```

The `query` is a HTML (XML) Pattern.
Patterns are valid HTML structures which has exact one placeholder `{}` for innerHTMLs or attributes.
For example,

```html
<p>{}</p>
```

```html
<p class="here">
    <q>{}</q>
</p>
```

`web-grep` outputs all text matching for `{}`.

**NOTE**: The original library `easy-scraper` uses tagged placeholders (e.g. `{{xxx}}`), but `web-grep` uses only `{}`.

