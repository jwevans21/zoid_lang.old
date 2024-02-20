# Zoid Language

The Zoid language is designed to be a simple, low-level programming language.

## Usage

### Installation

Currently the only way to install the Zoid compiler is to build it from source.

This requires:
- [Rust](https://www.rust-lang.org/tools/install)
- [LLVM](https://llvm.org/docs/GettingStarted.html)

```sh
cargo install --git https://github.com/jwevans21/zoid_lang.git --branch main
# OR
git clone https://github.com/jwevans21/zoid_lang.git
cd zoid_lang
cargo install --path crates/zoidc
```

### Compiling

```sh
zoidc <input_file>
```

This will compile the input file to an executable with the same name as the
input file, but without the `.zd` extension.

## Examples

> **Note:** This is a work in progress and some of the features described here
> may not be implemented yet.

```zoid
/*
 * The classic "Hello, World!" program.
 */

cimport "stdio.h";

fn main(): i32 {
    // Call the printf function from the C standard library.
    // The `c` prefix indicates that this is a C string.
    // These are guaranteed to be null-terminated.
    printf(c"Hello, World!\n");

    return 0;
}
```

```zoid
/*
 * A simple display of features.
 */

cimport "stdio.h";
// OR
// extern "C" fn printf(format: *u8, ...): i32;
// extern "C" fn scanf(format: *u8, ...): i32;


// A simple function to calculate the nth Fibonacci number.
fn fib(n: i32): i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

fn main(): i32 {
    let a = 42;

    if a == 42 {
        printf(c"The answer to life, the universe, and everything.\n");
    } else {
        printf(c"The answer to life, the universe, and everything is not 42.\n");
        printf(c"Also, your computer is broken.\n");
    }

    // Can also use `&&` instead of `and` and `||` instead of `or`.
    if a < 24 and a > 24 {
        printf(c"a is both greater than and less than 24\n");
    } else if a < 24 {
        printf(c"a is less than 24\n");
    } else {
        printf(c"a is greater than 24\n");
    }

    let i = 0;
    // Could also use `while i != 10` or `while !(i == 10)`.
    while not (i == 10) {
        printf(c"%d\n", i);
        i = i + 1;

        if i == 10 {
            i = i + 1;
        } else if i == 24 {
            break;
        }
    }

    let num = 0;
    scanf(c"%d", num.&);
    printf(c"You entered: %d\n", num);

    printf(c"The %d Fibonacci number is: %d\n", num, fib(num));

    return 0;
}
```