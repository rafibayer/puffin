
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use puffin::{Parser, PuffinParser, ast, interpreter};



pub fn fib_15_recursive(c: &mut Criterion) {
    let program = 
    r"
    fib = fn(n) {
        if (n == 0 || n == 1) {
            return n;
        }
    
    
        return fib(n-1) + fib(n-2);
    };
    
    return fib(15);
    ";

    let mut parsed = PuffinParser::parse(puffin::Rule::program, &program).unwrap();
    let prog_ast = ast::build_program(parsed.next().unwrap()).unwrap();

    c.bench_function("fib 15", |b| b.iter(|| {
        interpreter::eval(black_box(prog_ast.clone()))
    }));
}

pub fn fact_1_150_iterative(c: &mut Criterion) {
    let program = 
    r"
    ns = [1:151];
    res = [0];

    for (n in ns) {
        prod = 1;
        for (i in [1:n+1]) {
            prod *= i;
        }
        push(res, prod);
    }

    return res;
    ";

    let mut parsed = PuffinParser::parse(puffin::Rule::program, &program).unwrap();
    let prog_ast = ast::build_program(parsed.next().unwrap()).unwrap();

    c.bench_function("fact 1-150", |b| b.iter(|| {
        interpreter::eval(black_box(prog_ast.clone()))
    }));
}

pub fn first_500_primes(c: &mut Criterion) {
    let program =
    r"
    // 6k +- 1 primality test
    is_prime = fn(n) {
        if (n <= 3) {
            return n > 1;
        }

        if (n % 2 == 0 || n % 3 == 0) {
            return 0;
        }

        i = 5;
        while (pow(i, 2) <= n) {
            if (n % i == 0 || n % (i + 2) == 0) {
                return 0;
            }

            i += 6;
        }
        return 1;
    };

    N_PRIMES = 500;
    res = [N_PRIMES];
    idx = 0;

    n = 0;
    while (idx < N_PRIMES) {
        if (is_prime(n)) {
            res[idx] = n;
            idx += 1;
        }

        n += 1;
    }

    return res;
    ";
    let mut parsed = PuffinParser::parse(puffin::Rule::program, &program).unwrap();
    let prog_ast = ast::build_program(parsed.next().unwrap()).unwrap();

    c.bench_function("first 500 primes", |b| b.iter(|| {
        interpreter::eval(black_box(prog_ast.clone()))
    }));

}

criterion_group!(benches, fib_15_recursive, fact_1_150_iterative, first_500_primes);
criterion_main!(benches);