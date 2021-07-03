use std::io;
use std::io::Write;

use pest::Parser;

use crate::ast;
use crate::interpreter::repl::Repl;
use crate::interpreter::Value;
use crate::parser::Rule;
use crate::PuffinParser;

/// Starts the Puffin REPL
pub fn start_repl() -> ! {
    println!("Welcome to the Puffin REPL!");
    println!("(Ctrl-Z on newline to cancel input | Ctrl-C to exit)\n");

    // Repl environment
    let repl = Repl::new();
    let mut buffer = String::new();

    // REPL loop
    loop {
        // REPL read
        let bytes = readline(&mut buffer);
        if bytes == 0 {
            buffer.clear();
            println!("\n");
            continue;
        }

        if let Ok(mut stmt) = PuffinParser::parse(Rule::statement, &buffer) {

            let stmt_ast = ast::build_statement(stmt.next().unwrap()).unwrap();
            
            // REPL evaluate
            let res = repl
                .repl_statement(&stmt_ast)
                .unwrap()
                .unwrap_or(Value::Null);

            // REPL print
            if matches!(res, Value::Null) {
                println!();
            } else {
                println!("{}", res);
            }

            buffer.clear();
        }
    }
}

/// reads line from stdin into buffer, returning number of bytes read
fn readline(buffer: &mut String) -> usize {
    if !buffer.is_empty() {
        print!("... ");
    } else {
        print!(">>> ");
    }
    // flush stdout to display prompt
    io::stdout().flush().expect("Output Error");

    io::stdin().read_line(buffer).expect("Input Error")
}
