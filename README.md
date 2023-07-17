# splitters
SRX-based sentence splitter

## Compatibility with Rust regex
Some regex expressions might not be loaded because of syntax incompatibilities with Rust regex engine.
To avoid that, the SRX rules bundled with this package have been partially fixed to minimize this.
The `scripts/fix_regex.sh` contains the following fixes being applied:
  - Escape whitespace character at the begginging of `<afterbreak>`. For some reason the Rust xml parser is removing the space inside the rule for `<afterbreak> +</afterbreak>` so it ends up with the repetition operator missing its expression.
  - Unescape 'ظ' character for Farsi. Rust regex does not require it to be escaped.
  - `\Q` and `\E` expresssions are not supported, so removing them and escaping everything enclosed in it.
  - Escape dash before `\d` and `\p{...}` causing invalid range literal.

To see the loading errors, run `splitters` with `-v` option and use `-s` to provide one of the original SRX files in `data_orig` to see the fixed errors.
