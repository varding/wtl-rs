wtl-rs  [![Build status](https://ci.appveyor.com/api/projects/status/u04gc0xdi89g4huj?svg=true)](https://ci.appveyor.com/project/varding/wtl-rs)
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

Resource
=====
Urls below might be help.

* http://www.codeproject.com/Articles/3841/WTL-for-MFC-Programmers-Part-I-ATL-GUI-Classes
* http://www.codeproject.com/Articles/3867/WTL-for-MFC-Programmers-Part-II-WTL-GUI-Base-Class
* http://www.codeproject.com/Articles/3948/WTL-for-MFC-Programmers-Part-III-Toolbars-and-Stat
* http://www.codeproject.com/Articles/4028/WTL-for-MFC-Programmers-Part-IV-Dialogs-and-Contro
* http://www.codeproject.com/Articles/4029/WTL-for-MFC-Programmers-Part-V-Advanced-Dialog-UI