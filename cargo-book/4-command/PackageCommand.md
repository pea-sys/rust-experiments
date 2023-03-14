- cargo init
  ・・・cargo new はプロジェクトフォルダを作り、 cargo init はカレントディレクトリをプロジェクトフォルダにする)

```
C:\Users\user\projects\cargo-book\4-command\example-init>cargo init
     Created binary (application) package
```

- cargo install・・・ローカルにインストールする  
  ※Windows だと C:\Users\user\.cargo 配下にバイナリ配置

```
C:\Users\user\projects\cargo-book\4-command\example-init>cargo install ripgrep
    Updating crates.io index
  Installing ripgrep v13.0.0
   Compiling memchr v2.5.0
   Compiling winapi v0.3.9
   Compiling cfg-if v1.0.0
   Compiling log v0.4.17
   Compiling once_cell v1.17.1
   Compiling proc-macro2 v1.0.51
   Compiling regex-automata v0.1.10
   Compiling quote v1.0.23
   Compiling unicode-ident v1.0.6
   Compiling bstr v1.3.0
   Compiling syn v1.0.109
   Compiling aho-corasick v0.7.20
   Compiling regex-syntax v0.6.28
   Compiling winapi-util v0.1.5
   Compiling serde_derive v1.0.152
   Compiling serde v1.0.152
   Compiling same-file v1.0.6
   Compiling grep-matcher v0.1.6
   Compiling regex v1.7.1
   Compiling encoding_rs v0.8.32
   Compiling lazy_static v1.4.0
   Compiling serde_json v1.0.93
   Compiling unicode-width v0.1.10
   Compiling fnv v1.0.7
   Compiling globset v0.4.10
   Compiling textwrap v0.11.0
   Compiling encoding_rs_io v0.1.7
   Compiling termcolor v1.2.0
   Compiling thread_local v1.1.7
   Compiling bitflags v1.3.2
   Compiling memmap2 v0.5.10
   Compiling strsim v0.8.0
   Compiling bytecount v0.6.3
   Compiling ryu v1.0.12
   Compiling itoa v1.0.5
   Compiling grep-searcher v0.1.11
   Compiling clap v2.34.0
   Compiling atty v0.2.14
   Compiling base64 v0.20.0
   Compiling grep-printer v0.1.7
   Compiling grep-cli v0.1.7
   Compiling grep-regex v0.1.11
   Compiling walkdir v2.3.2
   Compiling ripgrep v13.0.0
   Compiling grep v0.2.11
   Compiling ignore v0.4.20
   Compiling bstr v0.2.17
   Compiling num_cpus v1.15.0
    Finished release [optimized + debuginfo] target(s) in 2m 33s
  Installing C:\Users\user\.cargo\bin\rg.exe
   Installed package `ripgrep v13.0.0` (executable `rg.exe`)
```

- cargo new

```
C:\Users\user\projects\cargo-book\4-command>cargo new foo
     Created binary (application) `foo` package
```

- cargo search

```
C:\Users\user\projects\cargo-book\4-command>cargo search serde
serde = "1.0.152"                       # A generic serialization/deserialization framework
discord_typed_interactions = "0.1.0"    # suppose you're working with discord slash commands and you want statically typed reques…
serde_json_experimental = "0.0.0"       # A JSON serialization file format
serde_valid = "0.13.0"                  # JSON Schema based validation tool using with serde.
alt_serde_json = "1.0.61"               # A JSON serialization file format
serde_json = "1.0.93"                   # A JSON serialization file format
serde_partiql = "1.1.65"                # A PartiQL data model serialization file format
deserr = "0.5.0"                        # Deserialization library with focus on error handling
serde-encrypt = "0.7.0"                 # Encrypts all the Serialize
serde-encrypt-core = "0.7.0"            # Encrypts all the Serialize
... and 4224 crates more (use --limit N to see more)
```

- cargo uninstall

```
C:\Users\user\projects\cargo-book\4-command\example-init>cargo uninstall ripgrep
    Removing C:\Users\user\.cargo\bin\rg.exe
```

- cargo login

```
C:\Users\user\projects\cargo-book\4-command>cargo login
please paste the token found on https://crates.io/me below
SECRET
       Login token for `crates.io` saved
```

- cargo owner

```
C:\Users\user\projects\cargo-book\4-command>cargo owner --list foo
    Updating crates.io index
carols10cents (Carol (Nichols || Goulding))
```

- cargo package

```
C:\Users\user\projects\cargo-book\4-command\foo>cargo package
warning: manifest has no description, license, license-file, documentation, homepage or repository.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
   Packaging foo v0.1.0 (C:\Users\user\projects\cargo-book\4-command\foo)
   Verifying foo v0.1.0 (C:\Users\user\projects\cargo-book\4-command\foo)
   Compiling foo v0.1.0 (C:\Users\user\projects\cargo-book\4-command\foo\target\package\foo-0.1.0)
    Finished dev [unoptimized + debuginfo] target(s) in 3.13s
    Packaged 4 files, 905.0B (705.0B compressed)
```

- cargo publish

```
C:\Users\user\projects\cargo-book\4-command\foo>cargo publish --dry-run
    Updating crates.io index
warning: manifest has no description, license, license-file, documentation, homepage or repository.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
   Packaging foo v0.1.0 (C:\Users\user\projects\cargo-book\4-command\foo)
   Verifying foo v0.1.0 (C:\Users\user\projects\cargo-book\4-command\foo)
   Compiling foo v0.1.0 (C:\Users\user\projects\cargo-book\4-command\foo\target\package\foo-0.1.0)
    Finished dev [unoptimized + debuginfo] target(s) in 2.23s
    Packaged 4 files, 905.0B (705.0B compressed)
   Uploading foo v0.1.0 (C:\Users\user\projects\cargo-book\4-command\foo)
warning: aborting upload due to dry run
```
