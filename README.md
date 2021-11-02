# rust-wc

Utility `wc` is a program that prints newline, word, and byte counts for each file.

Print  newline,  word,  and  byte  counts for each FILE, and a total line if more than one FILE is specified.  A word is a non-zero-length sequence of characters delimited by white space.

With no FILE, or when FILE is -, the process will exit. Third-party library: [clap](https://crates.io/crates/clap) will be used.  

It looks like this:

```bash
# rust-wc on 🌱 main [?] is 📦 v0.1.0 via 🦀 v1.56.1 
❯ cargo r somefile/does/exist
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rust-wc somefile/does/exist`
Os { code: 2, kind: NotFound, message: "No such file or directory" }

# rust-wc on 🌱 main [?] is 📦 v0.1.0 via 🦀 v1.56.1 
❯ cargo r Cargo.toml 
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rust-wc Cargo.toml`
 12  30  245  Cargo.toml

# rust-wc on 🌱 main [?] is 📦 v0.1.0 via 🦀 v1.56.1 
❯ cargo r Cargo.toml Cargo.toml 
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rust-wc Cargo.toml Cargo.toml`
 12  30  245  Cargo.toml
 12  30  245  Cargo.toml
 24  60  490  total

# rust-wc on 🌱 main [?] is 📦 v0.1.0 via 🦀 v1.56.1 
❯ cargo r Cargo.toml Cargo.toml -c
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rust-wc Cargo.toml Cargo.toml -c`
 245  Cargo.toml
 245  Cargo.toml
 490  total

# rust-wc on 🌱 main [?] is 📦 v0.1.0 via 🦀 v1.56.1 
❯ cargo r Cargo.toml Cargo.toml -m
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rust-wc Cargo.toml Cargo.toml -m`
 245  Cargo.toml
 245  Cargo.toml
 490  total

# rust-wc on 🌱 main [?] is 📦 v0.1.0 via 🦀 v1.56.1 
❯ cargo r Cargo.toml Cargo.toml -w
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rust-wc Cargo.toml Cargo.toml -w`
 30  Cargo.toml
 30  Cargo.toml
 60  total

# rust-wc on 🌱 main [?] is 📦 v0.1.0 via 🦀 v1.56.1 
❯ cargo r Cargo.toml Cargo.toml -l
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rust-wc Cargo.toml Cargo.toml -l`
 12  Cargo.toml
 12  Cargo.toml
 24  total

```

The projcet inspired by this [video](https://www.bilibili.com/video/BV14q4y1R7kB?from=search&seid=10579829407356805920&spm_id_from=333.337.0.0)

