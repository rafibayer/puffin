mod common;

#[cfg(test)]
mod test {

    use super::common::*;
    use std::collections::HashMap;

    // test programs that return literals
    #[test]
    fn test_value() {
        let tests = vec![
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
            (
                r#"return fn(){};"#,
                Value::Closure {
                    self_name: None,
                    args: Vec::new(),
                    block: Block { block: Vec::new() },
                    environment: Environment::new(),
                },
            ),
            (
                r#"return fn(a, b){};"#,
                Value::Closure {
                    self_name: None,
                    args: vec!["a", "b"].into_iter().map(str::to_string).collect(),
                    block: Block { block: Vec::new() },
                    environment: Environment::new(),
                },
            ),
            (
                r#"return fn(a) => a;"#,
                Value::Closure {
                    self_name: None,
                    args: vec!["a".to_string()],
                    block: Block {
                        block: vec![Statement {
                            statement: StatementKind::Return(Exp {
                                exp: vec![TermKind::Value(ValueKind::Name("a".to_string()))],
                                line: 1,
                            }),
                        }],
                    },
                    environment: Environment::new(),
                },
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
                Value::String("hello, world!".to_string())
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
                Value::from(15f64)
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
                Value::String("!dlrow ,olleh".to_string())
            )
        ];

        for (program, output) in tests {
            assert_eq!(run_program(program), output, "{}", program);
        }
    }
}
