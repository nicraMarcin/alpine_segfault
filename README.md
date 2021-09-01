
*building docker image*
```bash
$ docker build -t alpine:segfault .
Sending build context to Docker daemon  22.53kB
Step 1/11 : FROM rust:1.54.0 as build
 ---> 2781722edf1c
Step 2/11 : WORKDIR /app
 ---> Using cache
 ---> 5efc12e4289f
Step 3/11 : COPY . .
 ---> Using cache
 ---> dc5fbb9aa33b
Step 4/11 : RUN rustc --version     && cargo --version     && cargo test --workspace --verbose --release     && cargo build --release
 ---> Using cache
 ---> 1ab54d7fb29a
Step 5/11 : FROM alpine:3
 ---> 14119a10abf4
Step 6/11 : ENV TZ=Europe/Warsaw
 ---> Using cache
 ---> 86914dc8dd4f
Step 7/11 : WORKDIR /app
 ---> Using cache
 ---> 84d6b71c7f91
Step 8/11 : RUN apk --no-cache add libgcc gcompat
 ---> Using cache
 ---> e0c1e35c3dd2
Step 9/11 : COPY --from=build /app/target/release/gcore .
 ---> Using cache
 ---> f83d2ec135e7
Step 10/11 : COPY config/gurita.toml /etc/gurita/gurita.toml
 ---> Using cache
 ---> 9ea1eedaf6ad
Step 11/11 : CMD ["/app/gcore"]
 ---> Using cache
 ---> a506e515ff2f
Successfully built a506e515ff2f
Successfully tagged alpine:segfault
```
*running container and manually runnig app*
```bash
$ docker run --rm -it alpine:segfault sh
/app # ./gcore 
memory allocation of 39549127776 bytes failed
Aborted (core dumped)
/app # ./gcore 
Value of config set: /etc/gurita/gurita.toml
/app # ./gcore 
memory allocation of 45703548000 bytes failed
Aborted (core dumped)
/app # ./gcore 
Value of config set: /etc/gurita/gurita.toml
/app # ./gcore 
memory allocation of 39201623136 bytes failed
Aborted (core dumped)
/app # ./gcore 
Value of config set: /etc/gurita/gurita.toml
/app # ./gcore 
Value of config set: /etc/gurita/gurita.toml
/app # ./gcore 
Segmentation fault (core dumped)
/app # ./gcore 
Value of config set: /etc/gurita/gurita.toml
/app # ./gcore 
Value of config set: /etc/gurita/gurita.toml
/app # ./gcore 
memory allocation of 45245844576 bytes failed
Aborted (core dumped)
/app # 
```
**On alpine this doesnt print help**
```bash
/app # ldd gcore 
	/lib64/ld-linux-x86-64.so.2 (0x7fd68fb33000)
	libc.so.6 => /lib64/ld-linux-x86-64.so.2 (0x7fd68fb33000)
	ld-linux-x86-64.so.2 => /lib/ld-linux-x86-64.so.2 (0x7fd68fa6e000)
	libpthread.so.0 => /lib64/ld-linux-x86-64.so.2 (0x7fd68fb33000)
	libgcc_s.so.1 => /usr/lib/libgcc_s.so.1 (0x7fd68fa54000)
	libdl.so.2 => /lib64/ld-linux-x86-64.so.2 (0x7fd68fb33000)
/app # ./gcore -h
Value of config set: /etc/gurita/gurita.toml
/app # ./gcore -h
Segmentation fault (core dumped)
```

**This doesn't happen on debian:10-slim**
```bash
root@e6c01e911502:/app# ./gcore -h
gurita 0.1.0
GURITA

USAGE:
    gcore [FLAGS] [OPTIONS]

FLAGS:
    -d               Run as daemon
    -h, --help       Prints help information
    -v               Sets the level of verbosity (-v -vv -vvv)
    -V, --version    Prints version information

OPTIONS:
    -c, --config <FILE>    Sets custom conifg file. [default: /etc/gurita/gurita.toml]
    -p, --port <PORT>      Set listen port

```

**full output from building image wihtout using cache**
```bash
$ docker build -t alpine:segfault --no-cache .
Sending build context to Docker daemon  25.09kB
Step 1/11 : FROM rust:1.54.0 as build
 ---> 2781722edf1c
Step 2/11 : WORKDIR /app
 ---> Running in 6af7289ce1ca
Removing intermediate container 6af7289ce1ca
 ---> 46f4c3b37e02
Step 3/11 : COPY . .
 ---> b6f04e57c33d
Step 4/11 : RUN rustc --version     && cargo --version     && cargo test --workspace --verbose --release     && cargo build --release
 ---> Running in 200cef998148
rustc 1.54.0 (a178d0322 2021-07-26)
cargo 1.54.0 (5ae8d74b3 2021-06-22)
    Updating crates.io index
 Downloading crates ...
  Downloaded bitflags v1.3.2
  Downloaded ansi_term v0.11.0
  Downloaded vec_map v0.8.2
  Downloaded textwrap v0.11.0
  Downloaded atty v0.2.14
  Downloaded unicode-width v0.1.8
  Downloaded strsim v0.8.0
  Downloaded libc v0.2.101
  Downloaded clap v2.33.3
   Compiling libc v0.2.101
   Compiling unicode-width v0.1.8
   Compiling strsim v0.8.0
   Compiling vec_map v0.8.2
   Compiling ansi_term v0.11.0
   Compiling bitflags v1.3.2
     Running `rustc --crate-name build_script_build /usr/local/cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.101/build.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debug-assertions=off -C metadata=b6d682d86f9a1367 -C extra-filename=-b6d682d86f9a1367 --out-dir /app/target/release/build/libc-b6d682d86f9a1367 -L dependency=/app/target/release/deps --cap-lints allow`
     Running `rustc --crate-name unicode_width /usr/local/cargo/registry/src/github.com-1ecc6299db9ec823/unicode-width-0.1.8/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no --cfg 'feature="default"' -C metadata=08d0aaf781aabe41 -C extra-filename=-08d0aaf781aabe41 --out-dir /app/target/release/deps -L dependency=/app/target/release/deps --cap-lints allow`
     Running `rustc --crate-name strsim /usr/local/cargo/registry/src/github.com-1ecc6299db9ec823/strsim-0.8.0/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C metadata=7f5a090aa3e48636 -C extra-filename=-7f5a090aa3e48636 --out-dir /app/target/release/deps -L dependency=/app/target/release/deps --cap-lints allow`
     Running `rustc --crate-name vec_map /usr/local/cargo/registry/src/github.com-1ecc6299db9ec823/vec_map-0.8.2/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C metadata=c129dbcd7451c852 -C extra-filename=-c129dbcd7451c852 --out-dir /app/target/release/deps -L dependency=/app/target/release/deps --cap-lints allow`
     Running `rustc --crate-name ansi_term /usr/local/cargo/registry/src/github.com-1ecc6299db9ec823/ansi_term-0.11.0/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C metadata=c505298e282f1aef -C extra-filename=-c505298e282f1aef --out-dir /app/target/release/deps -L dependency=/app/target/release/deps --cap-lints allow`
     Running `rustc --crate-name bitflags --edition=2018 /usr/local/cargo/registry/src/github.com-1ecc6299db9ec823/bitflags-1.3.2/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no --cfg 'feature="default"' -C metadata=056e832fb17c54e9 -C extra-filename=-056e832fb17c54e9 --out-dir /app/target/release/deps -L dependency=/app/target/release/deps --cap-lints allow`
   Compiling textwrap v0.11.0
     Running `rustc --crate-name textwrap /usr/local/cargo/registry/src/github.com-1ecc6299db9ec823/textwrap-0.11.0/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C metadata=0792b3103f041915 -C extra-filename=-0792b3103f041915 --out-dir /app/target/release/deps -L dependency=/app/target/release/deps --extern unicode_width=/app/target/release/deps/libunicode_width-08d0aaf781aabe41.rmeta --cap-lints allow`
     Running `/app/target/release/build/libc-b6d682d86f9a1367/build-script-build`
     Running `rustc --crate-name libc /usr/local/cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.101/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C metadata=97afd5ddaa358af6 -C extra-filename=-97afd5ddaa358af6 --out-dir /app/target/release/deps -L dependency=/app/target/release/deps --cap-lints allow --cfg freebsd11 --cfg libc_priv_mod_use --cfg libc_union --cfg libc_const_size_of --cfg libc_align --cfg libc_core_cvoid --cfg libc_packedN --cfg libc_cfg_target_vendor`
   Compiling atty v0.2.14
     Running `rustc --crate-name atty /usr/local/cargo/registry/src/github.com-1ecc6299db9ec823/atty-0.2.14/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C metadata=d2539d886d0173e0 -C extra-filename=-d2539d886d0173e0 --out-dir /app/target/release/deps -L dependency=/app/target/release/deps --extern libc=/app/target/release/deps/liblibc-97afd5ddaa358af6.rmeta --cap-lints allow`
   Compiling clap v2.33.3
     Running `rustc --crate-name clap /usr/local/cargo/registry/src/github.com-1ecc6299db9ec823/clap-2.33.3/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no --cfg 'feature="ansi_term"' --cfg 'feature="atty"' --cfg 'feature="color"' --cfg 'feature="default"' --cfg 'feature="strsim"' --cfg 'feature="suggestions"' --cfg 'feature="vec_map"' -C metadata=ee0592fc7b6f382e -C extra-filename=-ee0592fc7b6f382e --out-dir /app/target/release/deps -L dependency=/app/target/release/deps --extern ansi_term=/app/target/release/deps/libansi_term-c505298e282f1aef.rmeta --extern atty=/app/target/release/deps/libatty-d2539d886d0173e0.rmeta --extern bitflags=/app/target/release/deps/libbitflags-056e832fb17c54e9.rmeta --extern strsim=/app/target/release/deps/libstrsim-7f5a090aa3e48636.rmeta --extern textwrap=/app/target/release/deps/libtextwrap-0792b3103f041915.rmeta --extern unicode_width=/app/target/release/deps/libunicode_width-08d0aaf781aabe41.rmeta --extern vec_map=/app/target/release/deps/libvec_map-c129dbcd7451c852.rmeta --cap-lints allow`
   Compiling gcore v0.1.0 (/app/gcore)
     Running `rustc --crate-name gcore --edition=2018 gcore/src/main.rs --error-format=json --json=diagnostic-rendered-ansi --emit=dep-info,link -C opt-level=3 -C embed-bitcode=no --test -C metadata=99cd7ad4b5667d79 -C extra-filename=-99cd7ad4b5667d79 --out-dir /app/target/release/deps -L dependency=/app/target/release/deps --extern clap=/app/target/release/deps/libclap-ee0592fc7b6f382e.rlib`
    Finished release [optimized] target(s) in 49.95s
     Running `/app/target/release/deps/gcore-99cd7ad4b5667d79`

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Compiling gcore v0.1.0 (/app/gcore)
    Finished release [optimized] target(s) in 0.74s
Removing intermediate container 200cef998148
 ---> 020e173486b5
Step 5/11 : FROM alpine:3
 ---> 14119a10abf4
Step 6/11 : ENV TZ=Europe/Warsaw
 ---> Running in 5863cac96b46
Removing intermediate container 5863cac96b46
 ---> f6dc4d04e624
Step 7/11 : WORKDIR /app
 ---> Running in 0075dd8bad5c
Removing intermediate container 0075dd8bad5c
 ---> 49e0bbcbceff
Step 8/11 : RUN apk --no-cache add libgcc gcompat
 ---> Running in cd7e03c5e648
fetch https://dl-cdn.alpinelinux.org/alpine/v3.14/main/x86_64/APKINDEX.tar.gz
fetch https://dl-cdn.alpinelinux.org/alpine/v3.14/community/x86_64/APKINDEX.tar.gz
(1/4) Installing musl-obstack (1.2.2-r0)
(2/4) Installing libucontext (1.1-r0)
(3/4) Installing gcompat (1.0.0-r2)
(4/4) Installing libgcc (10.3.1_git20210424-r2)
OK: 6 MiB in 18 packages
Removing intermediate container cd7e03c5e648
 ---> 60e3cbee9312
Step 9/11 : COPY --from=build /app/target/release/gcore .
 ---> 02d6b1ee859a
Step 10/11 : COPY config/gurita.toml /etc/gurita/gurita.toml
 ---> c54e2530637f
Step 11/11 : CMD ["/app/gcore"]
 ---> Running in 7191f754c065
Removing intermediate container 7191f754c065
 ---> 463c2de0d2d5
Successfully built 463c2de0d2d5
Successfully tagged alpine:segfault

```
