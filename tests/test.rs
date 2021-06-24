#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use puffin::{
        ast::{self, node::Block},
        interpreter::{self, value::Environment, Value},
        parser, Parser,
    };

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
            (r#"return [0];"#, Value::Array(vec![Value::Num(0f64); 0])),
            (r#"return [5];"#, Value::Array(vec![Value::Num(0f64); 5])),
            (r#"return {};"#, Value::Structure(HashMap::new())),
            (
                r#"return {fieldname: 123};"#,
                Value::Structure(
                    vec![("fieldname".to_string(), Value::Num(123f64))]
                        .into_iter()
                        .collect(),
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
                r#"return (1 + 1);"#,
                Value::Num(2f64)
            )
        ];

        for (program, output) in tests {
            assert_eq!(run_program(program), output, "{}", program);
        }
    }

    fn run_program(program: &str) -> Value {
        let parsed = parser::PuffinParser::parse(puffin::Rule::program, program)
            .unwrap()
            .next()
            .unwrap();
        let ast = ast::build_program(parsed).unwrap();
        interpreter::eval(ast).unwrap()
    }
}