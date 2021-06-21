#[cfg(test)]
mod test {
    use pest::Parser;

    use super::super::*;

    #[test]
    fn test_program() {
        let tests = vec![
            r"1;",
        ];

        for test in tests {
            let parsed = parse(test);
            ast(parsed).expect(test);

        }

    }

    fn parse<'i>(input: &'i str) -> Pair<'i, Rule> {
        PuffinParser::parse(Rule::program, input)
            .expect(&format!("Invalid test data: {}", input))
            .nth(0)
            .unwrap()
    }
}
