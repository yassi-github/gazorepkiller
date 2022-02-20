
## source build information

- operating system

```
Linux 5.4.0-100-generic #113-Ubuntu SMP Thu Feb 3 18:43:29 UTC 2022 x86_64 x86_64 x86_64 GNU/Linux
```

- tools or utilities versions
    - rust
        ```
        $ cargo --version
        cargo 1.58.0 (f01b232bc 2022-01-19)
        $ rustc -V
        rustc 1.58.1 (db9d1b20b 2022-01-20)
        $ rustup -V
        rustup 1.24.3 (ce5817a94 2021-05-31)
        
        $ rustup show

        installed targets for active toolchain
        --------------------------------------

        wasm32-unknown-unknown
        x86_64-unknown-linux-gnu

        active toolchain
        ----------------

        stable-x86_64-unknown-linux-gnu (default)
        rustc 1.58.1 (db9d1b20b 2022-01-20)
        ```
    - wasm-pack
        ```
        $ wasm-pack -V
        wasm-pack 0.10.2
        ```
    - gcc
        ```
        $ gcc --version
        gcc (Ubuntu 9.3.0-17ubuntu1~20.04) 9.3.0
        ```

- links to any tools or utilities that need to be downloaded
    - [rustup](https://www.rust-lang.org/tools/install)
        - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y`
    - [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
        - `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`
    - apt install
        - gcc
        - zip

- how to build
    1. run [build.sh](../../build.sh)
