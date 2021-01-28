# kindaOS
some sorta rust kernel i made in rust
(did i mention it's in rust)
# install/run
there's a `run.sh` script that should work if you have qemu

flash the precompiled binary in the github releases or the build result to a storage device you can boot off of and do some fuckin... bios shit to boot off the usb idk
# build 
1. cd into the kindaOS directory

2. switch to nightly and add the components `llvm-tools-preview` and `rust-src`
```sh
rustup override set nightly
rustup component add llvm-tools-preview
rustup component add rust-src
```

3. add the following to your `config.toml`
```toml
[unstable]
build-std-features = ["compiler-builtins-mem"]
build-std = ["core", "compiler_builtins", "alloc"]
```

4. `./build.sh`

(i hope)
