# proclist_rs

Add x86_64 target to build for windows x86_64
` rustup target add x86_64-pc-windows-gnu `

Using [cross-rs](https://github.com/cross-rs) to cross-compile to Windows x86_64
``` 
cargo install cross --git https://github.com/cross-rs/cross
cross build --target x86_64-pc-windows-gnu
```