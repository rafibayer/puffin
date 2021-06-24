use pest::error::Error;

extern crate pest;

#[derive(Parser)]
#[grammar = "puffin.pest"]
pub struct PuffinParser;

pub fn line_col(err: Error<Rule>) -> String {
    match err.line_col {
        pest::error::LineColLocation::Pos(p) => format!("line {}:{}", p.0, p.1),
        pest::error::LineColLocation::Span(start, end) => {
            format!("Span: {:?} -> {:?}", start, end)
        }
    }
}

#[cfg(test)]
mod test {

    use pest::Parser;

    use super::*;

    #[test]
    fn test_program() {
        PuffinParser::parse(
            Rule::program,
            r#"
        x = 5;
        if (x < 5) {
            print("hello world!");
        } else {
            for (x = 0; x < 5; x = x + 1) {
                print("nice!");
            }
        }
        "#,
        )
        .unwrap();
    }

    #[test]
    fn test_name() {
        let tests = vec![
            "valid",
            "valid1",
            "valid_1",
            "alpha_num3r1c_and_underscores",
            "a",
        ];
        for test in tests {
            let pairs = PuffinParser::parse(Rule::name, test).unwrap();
            let last = pairs.last().unwrap();
            assert_eq!(last.as_span().end_pos().pos(), test.len(), "{}", test);
        }
    }

    #[test]
    fn test_null() {
        let tests = vec![r"null", r"(null)"];

        for test in tests {
            let pairs = PuffinParser::parse(Rule::value, test).expect(test);
            let last = pairs.last().unwrap();
            assert_eq!(last.as_span().end(), test.len(), "{}", test);
        }
    }

    #[test]
    fn test_return() {
        let tests = vec![r"return 5", r"return arr[5]", r"return arr[5] + 5"];

        for test in tests {
            let pairs = PuffinParser::parse(Rule::return_statment, test).expect(test);
            let last = pairs.last().unwrap();
            assert_eq!(last.as_span().end_pos().pos(), test.len(), "{}", test);
        }
    }

    #[test]
    fn test_assign() {
        let tests = vec![
            r"x = 5",
            r"x[5] = 5",
            r"x[5] = 5 + 5",
            r"x[5 + 5] = 5 + 5",
            r"x[-1*(5 + 5)] = 5 + 5 + y[555]",
        ];

        for test in tests {
            let pairs = PuffinParser::parse(Rule::assign_statment, test).expect(test);
            let last = pairs.last().unwrap();
            assert_eq!(last.as_span().end_pos().pos(), test.len(), "{}", test);
        }
    }

    #[test]
    fn test_block() {
        let tests = vec![
            r#"{
            
        }
        "#,
            r#"{
            if (thing) {
                nice;
            }
        }
        "#,
            r#"{
            statement;
        }"#,
        ];
        for test in tests {
            PuffinParser::parse(Rule::block, test).expect(test);
        }
    }

    #[test]
    fn test_array_init() {
        let tests = vec![
            r"[5]",
            r"[5 + 5]",
            r"[call()]",
            r"[call(5)]",
            r"[call(5 + 5)]",
            r"[call(5, 5, 5)]",
            r"[call(5, a, arr[5])]",
        ];

        for test in tests {
            let pairs = PuffinParser::parse(Rule::array_init, test).unwrap();
            let last = pairs.last().unwrap();
            assert_eq!(last.as_span().end(), test.len(), "{}", test);
        }
    }

    #[test]
    fn test_array_index() {
        let tests = vec![
            r"arr[5+5]",
            r"arr[arr[1]]",
            r"arr[call(a, b, x[55]) + 123]",
            r"arr[call(a, b, x[55])]",
            r"(arr[a])[b + 123]",
            r"(arr[a][b])[c]",
            r"func(a, b)[c]",
            r"a[b][c][d][e][f][g]",
            r"[5][0]",
        ];

        for test in tests {
            let pairs = PuffinParser::parse(Rule::exp, test).expect(test);
            let last = pairs.last().unwrap();
            assert_eq!(last.as_span().end(), test.len(), "{}", test);
        }
    }

    #[test]
    fn test_unop() {
        let tests = vec![
            r"-5",
            r"-55",
            r"-(5)",
            r"-name",
            r"-arr[-5]",
            r#"-"string""#,
        ];

        for test in tests {
            let pairs = PuffinParser::parse(Rule::exp, test).expect(test);
            let last = pairs.last().unwrap();
            assert_eq!(last.as_span().end(), test.len(), "{}", test);
        }
    }

    #[test]
    fn test_function_def() {
        let tests = vec![
            r"fn(){}",
            r"fn(a){}",
            r"fn(a, b, c) {
                return a + b + c;
            }",
            r"fn(outer){
                inner = fn(a) {
                    return a;
                };
                return inner(outer);
            }",
        ];

        for test in tests {
            let pairs = PuffinParser::parse(Rule::function, test).expect(test);
            let last = pairs.last().unwrap();
            assert_eq!(last.as_span().end(), test.len(), "{}", test);
        }
    }

    #[test]
    fn test_function_call() {
        let tests = vec![
            r"func()",
            r"func(a)",
            r"func(a, b)",
            r"func(a, b, c)",
            r"func(1, 1+1, 1+1+1)",
            r"func(arr[1], arr[1]+1, arr[1+1]+1)",
            r"(func())()",
            r"arr[5]()",
            r"f()()()()()()()()",
            r"fn(a){}(b)",
        ];

        for test in tests {
            let pairs = PuffinParser::parse(Rule::exp, test).expect(test);
            let last = pairs.last().unwrap();
            assert_eq!(last.as_span().end(), test.len(), "{}", test);
        }
    }

    #[test]
    fn test_cond() {
        let tests = vec![
            r"if (a) {
                b;
            }",
            r"if (a == b) {
                c;
            }",
            r"if (a == b) {
                c;
            } else {
                d;
            }",
        ];

        for test in tests {
            let pairs = PuffinParser::parse(Rule::condnest, test).expect(test);
            let last = pairs.last().unwrap();
            assert_eq!(last.as_span().end(), test.len(), "{}", test);
        }
    }

    #[test]
    fn test_loop() {
        let tests = vec![
            r"while (cond) {}",
            r"while (cond) {a;}",
            r"for (i = 0; i < len(arr); i = i + 1) {arr[i];}",
            r"for (a; a; a) {a;}",
            r"for (a; a != null; a = next()) {a;}",
        ];

        for test in tests {
            let pairs = PuffinParser::parse(Rule::loopnest, test).expect(test);
            let last = pairs.last().unwrap();
            assert_eq!(last.as_span().end(), test.len(), "{}", test);
        }
    }

    #[test]
    fn test_structure() {
        let tests = vec![
            r#"{};"#,
            r#"{field:"value"};"#,
            r#"{nested:{inner:"value"}};"#,
        ];

        for test in tests {
            let pairs = PuffinParser::parse(Rule::statement, test).expect(test);
            let last = pairs.last().unwrap();
            assert_eq!(last.as_span().end(), test.len(), "{}", test);
        }
    }

    #[test]
    fn test_dot() {
        let tests = vec![
            r#"thing.property"#,
            r#"thing.property.inner_property"#,
            r#"thing.array[0].inner_property"#,
            r#"(thing).inner_property"#,
            r#"result().inner_property"#,
            r#"{literal:"value"}.literal"#,
        ];

        for test in tests {
            let pairs = PuffinParser::parse(Rule::exp, test).expect(test);
            let last = pairs.last().unwrap();
            assert_eq!(last.as_span().end(), test.len(), "{}", test);
        }
    }

    #[test]
    fn test_exp() {
        let tests = vec![
            r"1+1",
            r"1+1+1",
            r"1+(1+1)",
            r"-1+(1+1)",
            r"a+(1+1)",
            r"1+(a+1)",
            r"1+(1+a)",
            r"1-(1+1)/1",
        ];

        for test in tests {
            let pairs = PuffinParser::parse(Rule::exp, test).expect(test);
            let last = pairs.last().unwrap();
            assert_eq!(last.as_span().end(), test.len(), "{}", test);
        }
    }

    #[test]
    fn test_lambda() {
        let tests = vec![
            r#"fn() => 1"#,
            r#"fn() => (1 + 1)"#,
            r#"fn() => fn() => 1"#,
        ];

        for test in tests {
            let pairs = PuffinParser::parse(Rule::function, test).expect(test);
            let last = pairs.last().unwrap();
            assert_eq!(last.as_span().end(), test.len(), "{}", test);
        }
    }

    // test template
    #[ignore]
    #[test]
    fn test() {
        let tests = vec![r"case;"];

        for test in tests {
            let pairs = PuffinParser::parse(Rule::program, test).expect(test);
            let last = pairs.last().unwrap();
            assert_eq!(last.as_span().end(), test.len(), "{}", test);
        }
    }
}
