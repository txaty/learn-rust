# Hello world

To compile the program:

```bash
rustc main.rs
```

The ```main``` function is special: it is always the first code that runs in every executable Rust program.

Rust style is to indent with four spaces, not a tab.

```println!``` calls a Rust macro. If it had called a function instead it would be entered as ```println``` (without the ```!```).

We end the line with a semicolon(```;```), which indicates that this expression is over and the next one is ready to begin. Most lines of Rust code end with a semicolon.

Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed.

Compile Rust program with ```rustc```.
