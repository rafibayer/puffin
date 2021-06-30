
# `Puffin`
A simple, dynamic language interpreter written in Rust.

# Code Sample

```rs
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
`Puffin` Supports a mix of imperative and functional language features:

## First-Class Functions
`Puffin` functions are closures which capture their environment when evaluated, if bound to a name, functions may also be used recursively. `Puffin` functions are declared with the keyword `fn`, followed by any number of argument names in parens, and finally, the associated function body between curlies, or a single expression.

```rs
// Curried addition using closures
curry_add = fn(a) {
    return fn(b) => a + b;
};

curry_10 = curry_add(10);
print(curry_10(7));

// Output: 17
```
You may also pass functions like other types freely.

```rs
call_twice = fn(f) {
    f();
    f();
};

call_twice(fn() => println("Hello World"));
// Output: 
// Hello World
// Hello World
```

## Structures
`Puffin` Structures can be created and bound to names, arrays, or even other structure fields. Structure fields are dynamic and can be added ad-hoc.

```rs
// create a structure
user = {
    name: "Rafi",
    age: 22,
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
`puffin` allows arrays to have mixed types (this includes other arrays). See the [Builtins](##Builtins) section for more ways to manipulate arrays. Array indices have the initial value of null. 

Sized Initialization:
```rs
// create a new array of size 5
arr = [5];

// fill arr with 1-5
for (i = 0; i < len(arr); i += 1) {
    arr[i] = i + 1;
}

arr[len(arr)-1] = "Another type!";

println(arr);
// output: [1, 2, 3, 4, Another type!] 
```

Range Initialization:
```rs
arr = [0:10];
print(arr);
// output: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] 
```

## Loops
```rs
// standard for loop
for (i = 0; i < 10; i += 1) {
    println(i);
}

// for-in loop
for (i in [0:10]) {
    println(i);
}

// while-loop
i = 0;
while (i < 10) {
    println(i);
    i += 1;
}
```

## Builtins
`Puffin` currently supports the following builtin functions and constants, although more may be added soon. Builtins cannot be rebound (although they can be used as structure field names):
- `PI`: approximately `π`
- `true`: 1
- `false`: 0
- `EPSILON`: Rust `std::f64::EPSILON`
- `str(a)`: Returns string representation of `a`
- `len(a)`: Returns length of array, string, or structure `a`
- `print(...)`: prints elements of args delimited by spaces
- `println(...)`: prints elements of args delimited by spaces, followed by a newline
- `error(...)`: printlns args to sterr and exits with non-zero exit code
- `sin(a)`, `cos(a)`, `tan(a)`, `sqrt(a)`, `abs(a)`, `round(a)`: standard math functions
- `input_str(...)`, `input_num(...)`: prints args as prompt, parses next line from stdin as string or number.
- `push(a, b)`: pushes `b` onto the array `a`
- `pop(a)`: pops the last element `b`, off `a`, returning `b`
- `remove(a, i)`: removes and returns the element at index `i` in array `a`
- `insert(a, i, v)`: inserts element `v` into array `a` at index `i`
- `rand()`: returns a uniformly distributed random number between 0 and 1


## Types
`Puffin` supports the following types. All types are pass by value with the exception of `Array` and `Structure`, which are pass by refrence.
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
- Hash-table (and literals?)
- Runtime/AST error line numbers
- Imports/multi-file programs? standard library?

<hr>

```art lol
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
Puffin is the successor to my previous attempt at creating a language: [smp-lang](https://github.com/rafibayer/smp-lang).
