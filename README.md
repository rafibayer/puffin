
# `Puffin`
A simple, dynamic language interpreter written in Rust.

# Code Sample

```
// recursively computes factorial of n
fact = fn(n) {
    if (n < 2) {
        return 1;
    }

    return n * fact(n - 1);
};

// Take a number from stdin, and compute the factorial
print(fact(input_num("Factorial: ")));
```

# Features
`Puffin` Supports a mix of structural and dynamic language features:

## First-Class Functions
`Puffin` functions are closures which capture their environment when evaluated, if bound to a name, functions may also be used recursively. `Puffin` functions are declared with the keyword `fn`, followed by any number of argument names in parens, and finally, the associated function body between curlies.

```
// Curried addition using closures
curry_add = fn(a) {
    return fn(b) {
        return a + b;
    };
};

curry_10 = curry_add(10);
print(curry_10(7));

// Output: 17
```
You may also pass functions line other types freely.

```
call_twice = fn(f) {
    f();
    f();
};

call_twice(fn(){println("Hello World");});
// Output: 
// Hello World
// Hello World
```

## Structures
`Puffin` Structures can be created and bound to names, arrays, or even other structure fields. Structure fields are dynamic and can be added ad-hoc.

```
// create a structure
user = {
    name: "Rafi",
    Age: 22,
    contact: {
        github: "github.com/rafibayer",
        linkedin: "linkedin.com/in/rafael-bayer"
    }
};

// add a new field to the nested contact structure
user.contact.email = "rafibayer7@gmail.com";

// print the value of the 'contact' field
println(user.contact);

// output: 
// {github: github.com/rafibayer, linkedin: linkedin.com/in/rafael-bayer, email: rafibayer7@gmail.com} 
```

## Arrays
`puffin` Arrays are fixed sized data-structures, `puffin` allows arrays to have mixed types (this includes other arrays).

```
// create a new array of size 5
arr = [5];

// fill arr with 1-5
for (i = 0; i < len(arr); i = i + 1) {
    arr[i] = i + 1;
}

arr[len(arr)-1] = "Another type!";

println(arr);
// output: [1, 2, 3, 4, Another type!] 
```
## Builtins
`Puffin` currently supports the following builtin functions and constants, although more may be added soon. Builtins cannot be rebound (although they can be used as structure field names):
- `PI`: approx. pi
- `true`: 1
- `false`: 0
- `EPSILON`: Rust `std::f64::EPSILON`
- `len()`: Returns len of array, string, or structure
- `print()`: prints arbitrary number of args
- `println()`: printlns arbitrary number of args
- `error()`: printlns args to sterr and exits with non-zero exit code
- `sin(), cos(), tan(), sqrt(), abs()`: standard math functions
- `input_str(), input_num()`: prints args as prompt, parses next line from stdin as string or number.

## Types
`Puffin` supports the following types. All types are pass by value. If you modify a parameter to a function you will need to return in back to the caller, however this is trivial with dynamic structures and arrays which can be used to bundle return values if needed:
- `Null`
- `Num`
- `String`
- `Array`
- `Structure`
- `Closure`: Functions evaluate to closures
- `Builtin`: Used internally only, behaves like a regular function when called


## More
`Puffin` Also supports other standard features such as standard arithmetic, comparison, and logical operators. There is no boolean type, all numbers are evaluated as `true` unless they are `0`.

## Usage
To execute a source file, just pass it to the `puffin` cli.  

Example: `$ puffin program.puf`  

`puffin` also supports the following optional cli flags:
- `-parse`: Show the program parse tree before execution

- `-ast`: Show the program AST before execution

## Planned Features
- Array Resizing (automatic? via builtin?)
- Array Literals
- Hash-table (and literals?)
- String indexing
- More builtins
- Runtime/AST error line numbers
- Imports/multi-file programs? standard library?

<hr>

```
-----------------------------------------------------------
                            ,▄▄▄▄,
                        ▄██████████▄
                        ,███ ▀██ ▀██████▄
                        ███████████▓██████
                        ██████████▌ ▀▀▀▀▀▀
                        ████████████▄
                    ▄████████████████▄
                ▄▄████████████████████
            ,▄████████████████████████`
            ██████████████████████████▀
            ┌██████████████████████████`
            █████████████████████████▀
        ▄████████████████████████▀
        ,▄██████████████████████▀
        ▄█████████████████████▀
    ▀▀▀▀`         ███  ▐██▀
                    ▀▀▌  ▀▌
-----------------------------------------------------------

```