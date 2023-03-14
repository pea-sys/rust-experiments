# Build Command

- cargo help

```
C:\Users\user\projects\cargo-book\4-command>cargo help
Rust's package manager

Usage: cargo [+toolchain] [OPTIONS] [COMMAND]

Options:
  -V, --version             Print version info and exit
      --list                List installed commands
      --explain <CODE>      Run `rustc --explain CODE`
  -v, --verbose...          Use verbose output (-vv very verbose/build.rs output)
  -q, --quiet               Do not print cargo log messages
      --color <WHEN>        Coloring: auto, always, never
      --frozen              Require Cargo.lock and cache are up to date
      --locked              Require Cargo.lock is up to date
      --offline             Run without accessing the network
      --config <KEY=VALUE>  Override a configuration value
  -Z <FLAG>                 Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
  -h, --help                Print help information

Some common cargo commands are (see all commands with --list):
    build, b    Compile the current package
    check, c    Analyze the current package and report errors, but don't build object files
    clean       Remove the target directory
    doc, d      Build this package's and its dependencies' documentation
    new         Create a new cargo package
    init        Create a new cargo package in an existing directory
    add         Add dependencies to a manifest file
    remove      Remove dependencies from a manifest file
    run, r      Run a binary or example of the local package
    test, t     Run the tests
    bench       Run the benchmarks
    update      Update dependencies listed in Cargo.lock
    search      Search registry for crates
    publish     Package and upload this package to the registry
    install     Install a Rust binary. Default location is $HOME/.cargo/bin
    uninstall   Uninstall a Rust binary

See 'cargo help <command>' for more information on a specific command.
```

- cargo version

```
C:\Users\user\projects\cargo-book\4-command>cargo version
cargo 1.67.1 (8ecd4f20a 2023-01-10)
```

```
C:\Users\user\projects\cargo-book\4-command>cargo -Vv
cargo 1.67.1 (8ecd4f20a 2023-01-10)
release: 1.67.1
commit-hash: 8ecd4f20a9efb626975ac18a016d480dc7183d9b
commit-date: 2023-01-10
host: x86_64-pc-windows-msvc
libgit2: 1.5.0 (sys:0.16.0 vendored)
libcurl: 7.86.0-DEV (sys:0.4.59+curl-7.86.0 vendored ssl:Schannel)
os: Windows 10.0.19045 (Windows 10 Pro) [64-bit]
```

- cargo bench (unstable なので nightly を使用)

```
C:\Users\user\projects\cargo-book\4-command\example>cargo +nightly bench
       Fresh example v0.1.0 (C:\Users\user\projects\cargo-book\4-command\example)
warning: unused variable: `b`
  --> src\lib.rs:19:12
   |
19 |     fn foo(b: &mut Bencher) {
   |            ^ help: if this is intentional, prefix it with an underscore: `_b`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: `example` (lib test) generated 1 warning (run `cargo fix --lib -p example --tests` to apply 1 suggestion)
    Finished bench [optimized] target(s) in 0.02s
     Running `C:\Users\user\projects\cargo-book\4-command\example\target\release\deps\example-498232d95511364b.exe --bench`

running 2 tests
test tests::it_works ... ignored
test tests::foo ... bench:           0 ns/iter (+/- 0)

test result: ok. 0 passed; 0 failed; 1 ignored; 1 measured; 0 filtered out; finished in 0.00s
```

- cargo check

```
C:\Users\user\projects\cargo-book\4-command\example>cargo check
    Checking example v0.1.0 (C:\Users\user\projects\cargo-book\4-command\example)
     Running `rustc --crate-name example --edition=2021 src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 -C metadata=1a0bb593eb08049f -C extra-filename=-1a0bb593eb08049f --out-dir C:\Users\user\projects\cargo-book\4-command\example\target\debug\deps -C incremental=C:\Users\user\projects\cargo-book\4-command\example\target\debug\incremental -L dependency=C:\Users\user\projects\cargo-book\4-command\example\target\debug\deps`
    Finished dev [unoptimized + debuginfo] target(s) in 0.20s
```

- cargo clean

```
C:\Users\user\projects\cargo-book\4-command\example>cargo clean
    Removing C:\Users\user\projects\cargo-book\4-command\example\target
```

- cargo doc

```
C:\Users\user\projects\cargo-book\4-command\example>cargo doc
 Documenting example v0.1.0 (C:\Users\user\projects\cargo-book\4-command\example)
     Running `rustdoc --edition=2021 --crate-type lib --crate-name example src\lib.rs -o C:\Users\user\projects\cargo-book\4-command\example\target\doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=4465522ef84d117e -L dependency=C:\Users\user\projects\cargo-book\4-command\example\target\debug\deps --crate-version 0.1.0`
    Finished dev [unoptimized + debuginfo] target(s) in 1.64s
```

- cargo fetch

```
C:\Users\user\projects\cargo-book\4-command\example>cargo fetch
    Updating git repository `https://github.com/rust-lang/regex`
    Updating crates.io index
```

- cargo fix(vcs=none の場合、オプションが必要)

```
C:\Users\user\projects\cargo-book\4-command\example>cargo fix
error: no VCS found for this package and `cargo fix` can potentially perform destructive changes; if you'd like to suppress this error pass `--allow-no-vcs`
```

````
C:\Users\user\projects\cargo-book\4-command\example>cargo fix --allow-no-vcs
       Fresh regex-syntax v0.6.28 (https://github.com/rust-lang/regex#a9b2e023)
       Fresh memchr v2.5.0
       Fresh aho-corasick v0.7.20
       Fresh regex v1.7.1 (https://github.com/rust-lang/regex#a9b2e023)
    Checking example v0.1.0 (C:\Users\user\projects\cargo-book\4-command\example)
     Running `C:\Users\user\.rustup\toolchains\stable-x86_64-pc-windows-msvc\bin\cargo.exe rustc --crate-name example --edition=2021 src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 -C metadata=7f717e54afbc1a3c -C extra-filename=-7f717e54afbc1a3c --out-dir C:\Users\user\projects\cargo-book\4-command\example\target\debug\deps -C incremental=C:\Users\user\projects\cargo-book\4-command\example\target\debug\incremental -L dependency=C:\Users\user\projects\cargo-book\4-command\example\target\debug\deps --extern regex=C:\Users\user\projects\cargo-book\4-command\example\target\debug\deps\libregex-3dac163384353042.rmeta`
     Running `C:\Users\user\.rustup\toolchains\stable-x86_64-pc-windows-msvc\bin\cargo.exe rustc --crate-name example --edition=2021 src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 --test -C metadata=14eaf132e24b1f8f -C extra-filename=-14eaf132e24b1f8f --out-dir C:\Users\user\projects\cargo-book\4-command\example\target\debug\deps -C incremental=C:\Users\user\projects\cargo-book\4-command\example\target\debug\incremental -L dependency=C:\Users\user\projects\cargo-book\4-command\example\target\debug\deps --extern regex=C:\Users\user\projects\cargo-book\4-command\example\target\debug\deps\libregex-3dac163384353042.rmeta`
      Fixing src\lib.rs
      Fixing src\lib.rs
    Finished dev [unoptimized + debuginfo] target(s) in 0.44s
    ```
````

- cargo run

```
C:\Users\user\projects\cargo-book\4-command\example-bin>cargo run
   Compiling example-bin v0.1.0 (C:\Users\user\projects\cargo-book\4-command\example-bin)
     Running `rustc --crate-name example_bin --edition=2021 src\main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=d022a2eaa980f1b8 --out-dir C:\Users\user\projects\cargo-book\4-command\example-bin\target\debug\deps -C incremental=C:\Users\user\projects\cargo-book\4-command\example-bin\target\debug\incremental -L dependency=C:\Users\user\projects\cargo-book\4-command\example-bin\target\debug\deps`
    Finished dev [unoptimized + debuginfo] target(s) in 0.78s
     Running `target\debug\example-bin.exe`
Hello, world!
```

- cargo rustc(cargo build より細かい制御が可能)  
  https://stackoverflow.com/questions/60577598/whats-the-difference-between-cargos-build-and-rustc-commands

```
C:\Users\user\projects\cargo-book\4-command\example>cargo rustc
       Fresh regex-syntax v0.6.28 (https://github.com/rust-lang/regex#a9b2e023)
       Fresh memchr v2.5.0
       Fresh aho-corasick v0.7.20
       Fresh regex v1.7.1 (https://github.com/rust-lang/regex#a9b2e023)
   Compiling example v0.1.0 (C:\Users\user\projects\cargo-book\4-command\example)
     Running `rustc --crate-name example --edition=2021 src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=6a2362639ec99926 -C extra-filename=-6a2362639ec99926 --out-dir C:\Users\user\projects\cargo-book\4-command\example\target\debug\deps -C incremental=C:\Users\user\projects\cargo-book\4-command\example\target\debug\incremental -L dependency=C:\Users\user\projects\cargo-book\4-command\example\target\debug\deps --extern regex=C:\Users\user\projects\cargo-book\4-command\example\target\debug\deps\libregex-54ac53d7b0d1f310.rmeta`
    Finished dev [unoptimized + debuginfo] target(s) in 0.20s
```

- cargo rustdoc(cargo doc より細かい制御が可能)

```
C:\Users\user\projects\cargo-book\4-command\example>cargo rustdoc
       Fresh regex-syntax v0.6.28 (https://github.com/rust-lang/regex#a9b2e023)
       Fresh memchr v2.5.0
       Fresh aho-corasick v0.7.20
       Fresh regex v1.7.1 (https://github.com/rust-lang/regex#a9b2e023)
 Documenting example v0.1.0 (C:\Users\user\projects\cargo-book\4-command\example)
     Running `rustdoc --edition=2021 --crate-type lib --crate-name example src\lib.rs -o C:\Users\user\projects\cargo-book\4-command\example\target\doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=7e77117c89b0dfd0 -L dependency=C:\Users\user\projects\cargo-book\4-command\example\target\debug\deps --extern regex=C:\Users\user\projects\cargo-book\4-command\example\target\debug\deps\libregex-3dac163384353042.rmeta --crate-version 0.1.0`
    Finished dev [unoptimized + debuginfo] target(s) in 0.93s
```

- cargo test

```
C:\Users\user\projects\cargo-book\4-command\example>cargo test
       Fresh regex-syntax v0.6.28 (https://github.com/rust-lang/regex#a9b2e023)
       Fresh memchr v2.5.0
       Fresh aho-corasick v0.7.20
       Fresh regex v1.7.1 (https://github.com/rust-lang/regex#a9b2e023)
   Compiling example v0.1.0 (C:\Users\user\projects\cargo-book\4-command\example)
     Running `rustc --crate-name example --edition=2021 src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test -C metadata=29471c72c1c0a9f9 -C extra-filename=-29471c72c1c0a9f9 --out-dir C:\Users\user\projects\cargo-book\4-command\example\target\debug\deps -C incremental=C:\Users\user\projects\cargo-book\4-command\example\target\debug\incremental -L dependency=C:\Users\user\projects\cargo-book\4-command\example\target\debug\deps --extern regex=C:\Users\user\projects\cargo-book\4-command\example\target\debug\deps\libregex-54ac53d7b0d1f310.rlib`
    Finished test [unoptimized + debuginfo] target(s) in 0.84s
     Running `C:\Users\user\projects\cargo-book\4-command\example\target\debug\deps\example-29471c72c1c0a9f9.exe`

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

   Doc-tests example
     Running `rustdoc --edition=2021 --crate-type lib --crate-name example --test C:\Users\user\projects\cargo-book\4-command\example\src\lib.rs -L dependency=C:\Users\user\projects\cargo-book\4-command\example\target\debug\deps -L dependency=C:\Users\user\projects\cargo-book\4-command\example\target\debug\deps --extern example=C:\Users\user\projects\cargo-book\4-command\example\target\debug\deps\libexample-6a2362639ec99926.rlib --extern regex=C:\Users\user\projects\cargo-book\4-command\example\target\debug\deps\libregex-54ac53d7b0d1f310.rlib -C embed-bitcode=no --error-format human`

running 0 tests
```

- cargo report

```
C:\Users\user\projects\cargo-book\4-command\example>cargo report future-incompatibilities
The following warnings were discovered during the build. These warnings are an
indication that the packages contain code that will become an error in a
future release of Rust. These warnings typically cover changes to close
soundness problems, unintended or undocumented behavior, or critical problems
that cannot be fixed in a backwards-compatible fashion, and are not expected
to be in wide use.

Each warning should contain a link for more information on what the warning
means and how to resolve it.


To solve this problem, you can try the following approaches:


- Some affected dependencies have newer versions available.
You may want to consider updating them to a newer version to see if the issue has been fixed.

rental v0.5.5 has the following newer versions available: 0.5.6


- If the issue is not solved by updating the dependencies, a fix has to be
implemented by those dependencies. You can help with that by notifying the
maintainers of this problem (e.g. by creating a bug report) or by proposing a
fix to the maintainers (e.g. by creating a pull request):

  - rental@0.5.5
  - Repository: https://github.com/jpernst/rental
  - Detailed warning command: `cargo report future-incompatibilities --id 1 --package rental@0.5.5`

- If waiting for an upstream fix is not an option, you can use the `[patch]`
section in `Cargo.toml` to use your own version of the dependency. For more
information, see:
https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html#the-patch-section

The package `rental v0.5.5` currently triggers the following future incompatibility lints:
> warning: using an old version of `rental`
>    --> C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\rental-0.5.5\src\lib.rs:94:8
>     |
> 94  |         enum ProceduralMasqueradeDummyType {
>     |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
> ...
> 117 |     define_rental_traits!(32);
>     |     ------------------------- in this macro invocation
>     |
>     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
>     = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
>     = note: older versions of the `rental` crate will stop compiling in future versions of Rust; please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
>     = note: `#[allow(proc_macro_back_compat)]` on by default
>     = note: this warning originates in the macro `define_rental_traits` (in Nightly builds, run with -Z macro-backtrace for more info)
>
> warning: using an old version of `rental`
>    --> C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\rental-0.5.5\src\lib.rs:258:9
>     |
> 258 |               enum ProceduralMasqueradeDummyType {
>     |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
> ...
> 285 | / rental! {
> 286 | |     /// Example types that demonstrate the API generated by the rental macro.
> 287 | |     pub mod examples {
> 288 | |         use std::sync;
> ...   |
> 345 | |     }
> 346 | | }
>     | |_- in this macro invocation
>     |
>     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
>     = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
>     = note: older versions of the `rental` crate will stop compiling in future versions of Rust; please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
>     = note: `#[allow(proc_macro_back_compat)]` on by default
>     = note: this warning originates in the macro `rental` (in Nightly builds, run with -Z macro-backtrace for more info)
>
> warning: using an old version of `rental`
>    --> C:\Users\user\.cargo\registry\src\github.com-1ecc6299db9ec823\rental-0.5.5\src\lib.rs:258:9
>     |
> 258 |               enum ProceduralMasqueradeDummyType {
>     |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
> ...
> 350 | / rental! {
> 351 | |     /// Premade types for the most common use cases.
> 352 | |     pub mod common {
> 353 | |         use std::ops::DerefMut;
> ...   |
> 484 | |     }
> 485 | | }
>     | |_- in this macro invocation
>     |
>     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
>     = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
>     = note: older versions of the `rental` crate will stop compiling in future versions of Rust; please update to `rental` v0.5.6, or switch to one of the `rental` alternatives
>     = note: `#[allow(proc_macro_back_compat)]` on by default
>     = note: this warning originates in the macro `rental` (in Nightly builds, run with -Z macro-backtrace for more info)
>
```
