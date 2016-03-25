# multirust-version-name

Show the used Rust toolchain version name for the current working directory.

If you use [multirust](https://github.com/brson/multirust)
you may have toolchain overrides for certain directories.
Sometimes it is good to see which one that is.
Put it in your prompt and it is there when you need to know.


## Install

```
cargo install --git https://github.com/badboy/multirust-version-name
```

## Use

```
multirust-version-name
```

## Use in shell prompt

Add the following function to your `.bashrc` or `.zshrc`:

```shell
__rust_prompt() {
  local rustp=$(multirust-version-name)
  if [ -n "$rustp"  ] && [ "$rustp" != "default"  ]
  then
    echo " ${P_RUBY}$rustp${P_NO_COLOUR}"
  fi
}
```

Add the function to your `PS1`:

```shell
PS1="[%~ \$(__rust_prompt)%# "
```

Restart your shell and your done.

## Todo

* Search for override in upper directory.

## License

[MIT](LICENSE)
