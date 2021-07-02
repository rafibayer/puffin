pub(crate) mod common;

#[cfg(test)]
mod test {

    use super::common::*;
    use std::{cell::RefCell, collections::HashMap, rc::Rc};

    // test programs that return values
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
                Value::from([1, 0, 1, 0, 1, 0].iter().map(|e| Value::from(*e as f64)).collect::<Vec<Value>>())
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
                Value::Num((1_u128..25_u128).fold(1, |a, b| a * b) as f64)
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
                Value::Num((1_u128..23_u128).fold(1, |a, b| a * b) as f64)
            )
        ];

        for (program, output) in tests {
            assert_eq!(run_program(program), output, "{}", program);
        }
    }
}
