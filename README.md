wtl-rs  
=====
Windows WTL GUI library for Rust

Installation
=====
Clone project use commands below:
```
git clone https://github.com/varding/wtl-rs
cd wtl-rs
git submodule init
git submodule update
```

Example
=====
There is a simple example in examples/hello_dialog  directory

Use commands below to build and run:
```
cd .\examples\hello_dialog
cargo build
.\target\debug\hello-dialog.exe
```

Uncomment line 5,6 in examples/hello_dialog/src/main.rs if you don't need the console.

The project only be tested on win7 x64.