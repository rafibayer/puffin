//! Author: Rafael Bayer (2021)
//! This module defines integration tests for the Puffin language.

pub(crate) mod common;

#[cfg(test)]
mod test {

    use super::common::*;
    use std::collections::HashMap;

    /// This function is a large parameterized test that executes a large number
    /// of Puffin programs, comparing their output to an expected value.
    /// Test cases will fail if the output does not match, or if puffin encounters any errors.
    #[test]
    fn test_programs() {
        let tests = vec![
            (r#"return typeof(1) == typeof(2);"#, Value::Num(1f64)),
            (r#"return typeof(1) == NUM;"#, Value::Num(1f64)),
            (r#"return typeof(1) == typeof("mystr");"#, Value::Num(0f64)),
            (r#"return typeof(1) == STRING;"#, Value::Num(0f64)),
            (r#"return typeof(null) == NULL;"#, Value::Num(1f64)),
            (r#"return 1;"#, Value::Num(1f64)),
            (r#"return "";"#, Value::String("".to_string())),
            (
                r#"return "hello, world!";"#,
                Value::String("hello, world!".to_string()),
            ),
            (r#"return [0];"#, Value::from(Vec::new())),
            (r#"return [5];"#, Value::from(vec![Value::Null; 5])),
            (r#"return {};"#, Value::from(HashMap::new())),
            (
                r#"return {fieldname: 123};"#,
                Value::from(
                    vec![("fieldname".to_string(), Value::Num(123f64))]
                        .into_iter()
                        .collect::<HashMap<String, Value>>(),
                ),
            ),
            (r#"return (1 + 1);"#, Value::Num(2f64)),
            (
                r#"
                fact = fn(n) {
                    if (n < 2) {
                        return 1;
                    }

                    return n * fact(n - 1);
                };

                arr = [10];
                for (i = 0; i < 10; i += 1) {
                    arr[i] = fact(i + 1);
                }

                return arr;
            "#,
                Value::from(
                    vec![1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800]
                        .into_iter()
                        .map(|e| Value::from(e as f64))
                        .collect::<Vec<Value>>(),
                ),
            ),
            (
                r#"
                curry_add = fn(a) {
                    return fn(b) => a + b;
                };
                
                c10 = curry_add(10);
                return c10(7);
            "#,
                Value::from(17f64),
            ),
            (
                r#"return "hello, " + "world!";"#,
                Value::String("hello, world!".to_string()),
            ),
            (
                r#"
                s1 = {
                    k1: 1,
                    k2: 2,
                    k3: 3,
                    k4: [10]
                };
                
                a1 = [5];

                st = "abcdef";
                
                return len(s1) + len(a1) + len(st);
                "#,
                Value::from(15f64),
            ),
            (
                r#"
                rev = fn(string) {
                    res = "";
                    for (i = len(string)-1; i >= 0; i = i - 1) {
                        res += string[i];
                    }
                
                    return res;
                };
                
                return rev("hello, world!");
                "#,
                Value::String("!dlrow ,olleh".to_string()),
            ),
            (
                r#"
                is_palidrome = fn(s) {
    
                    for (i = 0; i < round((len(s) / 2) - 0.5); i += 1) {
                        if (s[i] != s[len(s) - i - 1]) {
                            return false;
                        }
                    }
                
                    return true;
                };
                
                strings = [0];
                push(strings, "1");
                push(strings, "12");
                push(strings, "121");
                push(strings, "122");
                push(strings, "1221");
                push(strings, "1222");
                
                for (i = 0; i < len(strings); i += 1) {
                    strings[i] = is_palidrome(strings[i]);
                }
                
                return strings;
                "#,
                Value::from(
                    [1, 0, 1, 0, 1, 0]
                        .iter()
                        .map(|e| Value::from(*e as f64))
                        .collect::<Vec<Value>>(),
                ),
            ),
            (
                r#"
                vec = [1:25];
                prod = 1;
                for (i = 0; i < len(vec); i += 1) {
                    prod *= vec[i];
                }

                return prod;
                "#,
                Value::Num((1_u128..25_u128).fold(1, |a, b| a * b) as f64),
            ),
            (
                r#"
                vec = [1:23];
                prod = 1;
                for (i in vec) {
                    prod *= i;
                }

                return prod;
                "#,
                Value::Num((1_u128..23_u128).fold(1, |a, b| a * b) as f64),
            ),
            (
                r#"
                fib = fn(n) {
                    if (n == 0 || n == 1) {
                        return n;
                    }
                
                
                    return fib(n-1) + fib(n-2);
                };
                
                return fib(15);
                "#,
                Value::from(610f64),
            ),
            (
                r#"
                stack = fn() => {
                    inner: [0],
                    push: fn(self, e) => push(self.inner, e)
                    pop: fn(self) => pop(self.inner)
                };
                
                s = stack();
                s.push(1);
                s.push(2);
                s.push(3);
                
                out = [0];
                for (i in [0:3]) {
                    push(out, s.pop());   
                }
                
                return out;
                "#,
                Value::from(
                    [3, 2, 1]
                        .iter()
                        .map(|e| Value::from(*e as f64))
                        .collect::<Vec<Value>>(),
                ),
            ),
            (
                r#"
                stack = fn() => {
                    inner: [0],
                    push: fn(self, e) => push(self.inner, e)
                    pop: fn(self) => pop(self.inner)
                };
                
                two_stacks = {
                    one: stack(),
                    two: stack()
                };
                
                
                two_stacks.one.push(1);
                two_stacks.two.push(2);
                
                if (two_stacks.one.pop() != 1) {
                    error("failed 1");
                }
                if (two_stacks.two.pop() != 2) {
                    error("failed 2");
                }
                "#,
                Value::Null,
            ),
            (
                r#"
                o = fn() => {
                    i: {
                        f: fn(self) => self.i1,
                        i1: 2
                    },
                    o1: 1,
                    g: fn(self) => self.o1 + self.i.f()

                };

                o_ = o();
                return o_.g();
                "#,
                Value::Num(3f64),
            ),
            (
                r#"
                fact_solver = fn() => {
                    f: fn(self, n) {
                        if (n < 2) {
                            return 1;
                        }

                        return n * self.f(n-1);
                    }
                };



                return fact_solver().f(6);
                "#,
                Value::Num(720f64),
            ),
            (
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
                
                for (i in [0:250]) {
                    h.put(i, str(i));
                }
                
                for (i in [0:250]) {
                    if (!h.contains_key(i)) {
                        error("didn't contain", i);
                    }
                    if (h.get(i) != str(i)) {
                        error("wrong value for", i, ":", h.get(i));
                    }
                }
                
                for (i in [0:250]) {
                    h.remove(i);
                }
                "#,
                Value::Null,
            ),
        ];

        for (program, output) in tests {
            assert_eq!(run_program(program), output, "{}", program);
        }
    }
}
