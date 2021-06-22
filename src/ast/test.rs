#[cfg(test)]
mod test {
    use pest::Parser;
    use crate::PuffinParser;

    use super::super::*;

    #[test]
    fn test_program() {
        let tests = vec![
            r"1;",
            r"x = 1;",
            r"x = x;",
            r"x[x] = x;",
            r"x[x] = func(x);",
            r"x[func(x)] = func(x);",
            r#"
            x = [5];
            for (i = 0; i < len(x); i = i + 1) {
                x[i] = factorial(i+1);
            }

            return x;
            "#


        ];

        for test in tests {
            let parsed = parse(test);
            build_program(parsed).expect(test);
        }

    }

    fn parse<'i>(input: &'i str) -> Pair<'i, Rule> {
        PuffinParser::parse(Rule::program, input)
            .expect(&format!("Invalid test data: {}", input))
            .nth(0)
            .unwrap()
    }
}
