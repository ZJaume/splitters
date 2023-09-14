# splitte(.)rs
*There's still some work pending to make this usable*

A CLI for Rust [SRX](https://github.com/bminixhofer/srx) implementation as a Python package.

## Installation
Installing from source needs Rust Cargo to be installed. Install it with your package manager or with https://rustup.rs/.

Then, clone the repo and install it as any other Python package:
```
git clone https://github.com/ZJaume/splitters
pip install ./splitters
```

## Usage
Example usage
```bash
echo "Yes this is a sentence. Another one." | splitters -i /dev/stdin --output /dev/stdout
```
```
Yes this is a sentence.
Another one.
```

Full list of parameters:
```
splitters 0.1.0

USAGE:
    splitters [OPTIONS] --input <INPUT> --output <OUTPUT>

OPTIONS:
    -h, --help                   Print help information
    -i, --input <INPUT>
    -l, --language <LANGUAGE>    ISO-639-1, 2 char language code [default: en]
    -o, --output <OUTPUT>
    -s, --srxfile <SRXFILE>      [default: ]
    -v, --verbose
    -V, --version                Print version information
```

## Compatibility with Rust regex
Some regex expressions might not be loaded because of syntax incompatibilities with Rust regex engine.
To avoid that, the SRX rules bundled with this package have been partially fixed to minimize this.
The `scripts/fix_regex.sh` contains the following fixes being applied:
  - Escape whitespace character at the begginging of `<afterbreak>`. For some reason the Rust xml parser is removing the space inside the rule for `<afterbreak> +</afterbreak>` so it ends up with the repetition operator missing its expression.
  - Unescape 'ظ' character for Farsi. Rust regex does not require it to be escaped.
  - `\Q` and `\E` expresssions are not supported, so removing them and escaping everything enclosed in it.
  - Escape dash before `\d` and `\p{...}` causing invalid range literal.

To see the loading errors, run `splitters` with `-v` option and use `-s` to provide one of the original SRX files in `data_orig` to see the fixed errors.
