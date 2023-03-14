# Manifest Command

- cargo add

```
C:\Users\user\projects\cargo-book\4-command\example>cargo add regex
    Updating git repository `https://github.com/rust-lang/regex`
      Adding regex (git) to dependencies.
```

```
C:\Users\user\projects\cargo-book\4-command\example>cargo add --dev trybuild
    Updating crates.io index
      Adding trybuild v1.0.77 to dev-dependencies.
             Features:
             - diff
             - dissimilar

```

```
C:\Users\user\projects\cargo-book\4-command\example>cargo add nom@5
    Updating crates.io index
      Adding nom v5 to dependencies.
             Features as of v5.0.0:
             + alloc
             + lexical
             + lexical-core
             + std
             - lazy_static
             - regex
             - regexp
             - regexp_macros
```

-F は festures 指定（ドキュメントに説明がなかったのでプルリクしておきました)

```
C:\Users\user\projects\cargo-book\4-command\example>cargo add serde serde_json -F serde/derive
    Updating crates.io index
      Adding serde v1.0.152 to dependencies.
             Features:
             + derive
             + serde_derive
             + std
             - alloc
             - rc
             - unstable
      Adding serde_json v1.0.93 to dependencies.
             Features:
             + std
             - alloc
             - arbitrary_precision
             - float_roundtrip
             - indexmap
             - preserve_order
             - raw_value
             - unbounded_depth
```

- cargo generate-lockfile

```
C:\Users\user\projects\cargo-book\4-command\example>cargo generate-lockfile
    Updating crates.io index
    Updating git repository `https://github.com/rust-lang/regex`
```

- cargo-locate-project

```
C:\Users\user\projects\cargo-book\4-command\example>cargo locate-project
{"root":"C:\\Users\\user\\projects\\cargo-book\\4-command\\example\\Cargo.toml"}
```

- cargo metadata

```
C:\Users\user\projects\cargo-book\4-command\example>cargo metadata
warning: please specify `--format-version` flag explicitly to avoid compatibility problems
  Downloaded static_assertions v1.1.0
  Downloaded arrayvec v0.5.2
  Downloaded trybuild v1.0.77
  Downloaded nom v5.1.2
  Downloaded basic-toml v0.1.1
  Downloaded lexical-core v0.7.6
  Downloaded winapi-x86_64-pc-windows-gnu v0.4.0
  Downloaded winapi-i686-pc-windows-gnu v0.4.0
  Downloaded 8 crates (6.6 MB) in 1m 01s (largest was `winapi-x86_64-pc-windows-gnu` at 2.9 MB)
{"packages":[{"name":"aho-corasick","version":"0.7.20","id":"aho-corasick 0.7.20 (registry+https://github.com/rust-lang/crates.io-index)","license":"Unlicense OR MIT","license_file":null,"description":"Fast multiple substring searching.","source":"registry+https://github.com/rust-lang/crates.io-index","dependencies":[{"name":"memchr","source":"registry+https://github.com/rust-lang/crates.io-index","req":"^2.4.0","kind":null,"rename":null,"optional":false,"uses_default_features":false,"features":[],"target":null,"registry":null}],"targets":[{"kind":["lib"],
・・・(長いので略)
```

- cargo pkgid

```
C:\Users\user\projects\cargo-book\4-command\example>cargo pkgid
file:///C:/Users/user/projects/cargo-book/4-command/example#0.1.0
```

- cargo remove

```
C:\Users\user\projects\cargo-book\4-command\example>cargo remove regex
    Removing regex from dependencies
```

- cargo tree

```
C:\Users\user\projects\cargo-book\4-command\example>cargo tree
example v0.1.0 (C:\Users\user\projects\cargo-book\4-command\example)
├── nom v5.1.2
│   ├── lexical-core v0.7.6
│   │   ├── arrayvec v0.5.2
│   │   ├── bitflags v1.3.2
│   │   ├── cfg-if v1.0.0
│   │   ├── ryu v1.0.12
│   │   └── static_assertions v1.1.0
│   └── memchr v2.5.0
│   [build-dependencies]
│   └── version_check v0.9.4
├── rental v0.5.5
│   ├── rental-impl v0.5.5 (proc-macro)
│   │   ├── proc-macro2 v1.0.51
│   │   │   └── unicode-ident v1.0.6
│   │   ├── quote v1.0.23
│   │   │   └── proc-macro2 v1.0.51 (*)
│   │   └── syn v1.0.109
│   │       ├── proc-macro2 v1.0.51 (*)
│   │       ├── quote v1.0.23 (*)
│   │       └── unicode-ident v1.0.6
│   └── stable_deref_trait v1.2.0
├── serde v1.0.152
│   └── serde_derive v1.0.152 (proc-macro)
│       ├── proc-macro2 v1.0.51 (*)
│       ├── quote v1.0.23 (*)
│       └── syn v1.0.109 (*)
└── serde_json v1.0.93
    ├── itoa v1.0.5
    ├── ryu v1.0.12
    └── serde v1.0.152 (*)
[dev-dependencies]
└── trybuild v1.0.77
    ├── basic-toml v0.1.1
    │   └── serde v1.0.152 (*)
    ├── glob v0.3.1
    ├── once_cell v1.17.1
    ├── serde v1.0.152 (*)
    ├── serde_derive v1.0.152 (proc-macro) (*)
    ├── serde_json v1.0.93 (*)
    └── termcolor v1.2.0
        └── winapi-util v0.1.5
            └── winapi v0.3.9
```

- cargo update

```
C:\Users\user\projects\cargo-book\4-command\example>cargo update
    Updating crates.io index
```

- cargo vendor

```
C:\Users\user\projects\cargo-book\4-command\example>cargo vendor
   Vendoring arrayvec v0.5.2 (C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\arrayvec-0.5.2) to vendor\arrayvec
   Vendoring basic-toml v0.1.1 (C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\basic-toml-0.1.1) to vendor\basic-toml
   Vendoring bitflags v1.3.2 (C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\bitflags-1.3.2) to vendor\bitflags
   Vendoring cfg-if v1.0.0 (C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\cfg-if-1.0.0) to vendor\cfg-if
   Vendoring glob v0.3.1 (C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\glob-0.3.1) to vendor\glob
   Vendoring itoa v1.0.5 (C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\itoa-1.0.5) to vendor\itoa
   Vendoring lexical-core v0.7.6 (C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\lexical-core-0.7.6) to vendor\lexical-core
   Vendoring memchr v2.5.0 (C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\memchr-2.5.0) to vendor\memchr
   Vendoring nom v5.1.2 (C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\nom-5.1.2) to vendor\nom
   Vendoring once_cell v1.17.1 (C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\once_cell-1.17.1) to vendor\once_cell
   Vendoring proc-macro2 v1.0.51 (C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\proc-macro2-1.0.51) to vendor\proc-macro2
   Vendoring quote v1.0.23 (C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\quote-1.0.23) to vendor\quote
   Vendoring rental v0.5.5 (C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\rental-0.5.5) to vendor\rental
   Vendoring rental-impl v0.5.5 (C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\rental-impl-0.5.5) to vendor\rental-impl
   Vendoring ryu v1.0.12 (C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\ryu-1.0.12) to vendor\ryu
   Vendoring serde v1.0.152 (C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\serde-1.0.152) to vendor\serde
   Vendoring serde_derive v1.0.152 (C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\serde_derive-1.0.152) to vendor\serde_derive
   Vendoring serde_json v1.0.93 (C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\serde_json-1.0.93) to vendor\serde_json
   Vendoring stable_deref_trait v1.2.0 (C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\stable_deref_trait-1.2.0) to vendor\stable_deref_trait
   Vendoring static_assertions v1.1.0 (C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\static_assertions-1.1.0) to vendor\static_assertions
   Vendoring syn v1.0.109 (C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\syn-1.0.109) to vendor\syn
   Vendoring termcolor v1.2.0 (C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\termcolor-1.2.0) to vendor\termcolor
   Vendoring trybuild v1.0.77 (C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\trybuild-1.0.77) to vendor\trybuild
   Vendoring unicode-ident v1.0.6 (C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\unicode-ident-1.0.6) to vendor\unicode-ident
   Vendoring version_check v0.9.4 (C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\version_check-0.9.4) to vendor\version_check
   Vendoring winapi v0.3.9 (C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\winapi-0.3.9) to vendor\winapi
   Vendoring winapi-i686-pc-windows-gnu v0.4.0 (C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\winapi-i686-pc-windows-gnu-0.4.0) to vendor\winapi-i686-pc-windows-gnu
   Vendoring winapi-util v0.1.5 (C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\winapi-util-0.1.5) to vendor\winapi-util
   Vendoring winapi-x86_64-pc-windows-gnu v0.4.0 (C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\winapi-x86_64-pc-windows-gnu-0.4.0) to vendor\winapi-x86_64-pc-windows-gnu
To use vendored sources, add this to your .cargo/config.toml for this project:

[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
```

- cargo verify-project

```
C:\Users\user\projects\cargo-book\4-command\example>cargo verify-project
{"success":"true"}
```
