## Overview
`whilex` is a rust based based tree-walk interpreter for a Turing-complete programming language consisting entirely (as the name suggests) of while-loops.

The semantics and syntax of the language are based on the specifications provided by Philipp Hennig in his [lecture on computability theory](https://www.youtube.com/watch?v=XdWvN1o9xas).

## Usage
This is a quick overview over the features and syntax of the language.
### Creating a variable

```javascript
// Comments have a C-like syntax
x0 < 10; // Assign 10 to x0
```
The language only supports positive integers. However, the integers can be arbitrarily long. All variables are predefined to be `0`, so there is no need to declare them.

### Arithmetic

```javascript
x0  < x1 - 10;  // Assigns x1 - 10 to x0
box < 10 + box; // Increments box by 10
```
The language only supports unnested expressions and only `+` and `-` operators. 
If you subtract two values `a-b` where `b>a`, the result will be 0.

### Loops
```javascript
// Decrements `value` until it is zero
while value != 0 {
  value < value - 1;
}
```
The only control-flow statement supported is a `while`-loop. The condition of this loop can only be `var != 0` where `var` is some variable.

### Procedures
```javascript
// Declares a procedure called `mult` that multiplies
// numbers `a` and `b`.
procedure mult {
    count < a;
    a < 0;
    while count != 0 {
        count < count - 1;
        a < a + b;
    }
}

a < 10;
b < 20;
// This is how you call a procedure
mult; // `a` will now hold the value 200
```
There is only one global scope. This is why procedures do neither take any values as arguments nor return anything. A procedure has to be defined before it is used.

### I/O
Every sophisticated language needs I/O, so `whilex` especially does.

When you run your program through the terminal, you specify the file-path of a `.whl` file and optionally a list of initial parameters. These will be stored to variables `x0` to `xn`.

The output of the following program will thus be `30` in this case.

```javascript
x0 < x3; // Assigning x3 to x0
```
```sh
whilex path/file.whl 10 20 30
```

That is basically all there is to know :)


## Getting started
So how do you get your hands on this fancy language?
1. Make sure you have cargo installed.
2. Install `whilex` with cargo
   ```sh
   cargo install --git https://github.com/Uzzlius/whilex.git
   ```
  3. Make sure that `~/.cargo/bin/` is in your systems `PATH` environment variable, or otherwise specify the full path going forward
4. Run a `.whl` file using
   ```sh
   whilex path/file.whl
   ```
