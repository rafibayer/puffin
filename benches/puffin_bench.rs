//! Author: Rafael Bayer (2021)
//! This module contains benchmark tests for the Puffin Language.
//! Specifically, these tests are meant to measure the speed of the interpreter,
//! rather than the speed of the parser or AST generator.
//! Because of this, these tests parse and build the AST as setup, only
//! measuring the actual execution of the program itself.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use puffin::{Parser, PuffinParser, ast, interpreter};


/// Recursively compute the 15th number in the fibonacci sequence
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
        interpreter::eval(black_box(&prog_ast))
    }));
}

/// Iteratively compute the factorial of every integer, 1 through 150 
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
        interpreter::eval(black_box(&prog_ast))
    }));
}

/// Compute the first 500 prime numbers
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
        interpreter::eval(black_box(&prog_ast))
    }));

}

/// Put, get, and remove 1000 key-value pairs from a 
/// hashmap-like datastructure implemented in puffin
pub fn puffin_hashmap_struct(c: &mut Criterion) {
    let program = 
    r#"
    pair_ = fn(k, v) => {
        k:k,
        v:v
    };
    
    hashmap = fn() => {
    
        buckets_: fn() {
            arr=[1];
            arr[0]=[0];
            return arr;
        }(),
        size: 0,
    
        contains_key: fn(self, k) {
            search_bucket = self.hash_(k) % len(self.buckets_);
            for (kv in self.buckets_[search_bucket]) {
                if (kv.k == k) {
                    return true;
                }
            }
    
            return false;
        }
    
        put: fn(self, k, v) {
            dest_bucket = self.hash_(k) % len(self.buckets_);
            
            if (!self.contains_key(k)) {
                self.size += 1;
                push(self.buckets_[dest_bucket], pair_(k, v));
                if (self.size / len(self.buckets_) >= self.RESIZE_FACTOR_) {
                    self.resize_();
                }
                return null;
            }
    
            for (i = 0; i < len(self.buckets_[dest_bucket]); i += 1) {
                if (self.buckets_[dest_bucket][i].k == k) {
                    self.buckets_[dest_bucket][i] = pair_(k, v);
                    return null;
                }
            }
    
            error("unreachable!");
        },
    
        get: fn(self, k) {
            search_bucket = self.hash_(k) % len(self.buckets_);
            for (kv in self.buckets_[search_bucket]) {
                if (kv.k == k) {
                    return kv.v;
                }
            }
    
            error("Key not found:", k);
        },
    
        remove: fn(self, k) {
            search_bucket = self.hash_(k) % len(self.buckets_);
            for (i = 0; i < len(self.buckets_[search_bucket]); i += 1) {
                if (self.buckets_[search_bucket][i].k == k) {
                    removed = remove(self.buckets_[search_bucket], i);
                    self.size -= 1;
                    return removed;
                }
            }
    
            error("Key not found:", k);
        }
    
        resize_: fn(self) {
    
            new_buckets_ = [len(self.buckets_) * 2];
            for (b in [0:len(new_buckets_)]) {
                new_buckets_[b] = [0];
            }
    
            for (old in self.buckets_) {
                for (kv in old) {
                    dest_bucket = self.hash_(kv.k) % len(new_buckets_);
                    push(new_buckets_[dest_bucket], kv);
                }
            }
    
            self.buckets_ = new_buckets_;
        }
    
        hash_: fn(k) => k,
        RESIZE_FACTOR_: 0.75
    };

    h = hashmap();
    
    for (i in [0:1000]) {
        h.put(i, str(i));
    }
    
    for (i in [0:1000]) {
        if (!h.contains_key(i)) {
            error("didn't contain", i);
        }
        if (h.get(i) != str(i)) {
            error("wrong value for", i, ":", h.get(i));
        }
    }
    
    for (i in [0:1000]) {
        h.remove(i);
    }
    "#;
    let mut parsed = PuffinParser::parse(puffin::Rule::program, &program).unwrap();
    let prog_ast = ast::build_program(parsed.next().unwrap()).unwrap();

    c.bench_function("puffin hashmap 0:1000", |b| b.iter(|| {
        interpreter::eval(black_box(&prog_ast))
    }));

}

criterion_group!(benches, fib_15_recursive, fact_1_150_iterative, first_500_primes, puffin_hashmap_struct);
criterion_main!(benches);