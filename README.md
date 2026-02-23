# su(b command t)ree

`suree` visualizes the tree of subcommands for a given command.

## Usage

`suree` execute the specified command **ACTUALLY** with flag `--help`, and recursively explores its subcommands.

```bash
$ suree helm
helm
├── completion
│   ├── bash
│   ├── fish
│   ├── powershell
│   └── zsh
├── create
├── dependency
│   ├── build
│   ├── list
│   └── update
├── diff
│   ├── completion
│   │   ├── bash
│   │   ├── fish
│   │   ├── powershell
│   │   └── zsh
│   ├── release
│   ├── revision
│   ├── rollback
│   ├── upgrade
│   └── version
...snip
├── verify
└── version
```

## Install

Download a pre-built binary from [Releases](https://github.com/nstgt/suree/releases) and put it in your `$PATH`.

Alternatively you can `git clone https://github.com/nstgt/suree` and run `cargo build --release` in the directory.

## License

MIT. See [LICENSE.txt](./LICENSE.txt).
