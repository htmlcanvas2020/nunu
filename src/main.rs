use std::{collections::HashMap, io::Write};

mod language;
mod lite_parse;
mod parse;

use language::{CommandDefinition, ExpressionShape, Parameter, Scope};
use parse::parse;
use std::io;

fn main() {
    let mut input = String::new();
    loop {
        print!("> ");
        let _ = std::io::stdout().flush();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() == "quit" || input.trim() == "exit" {
                    break;
                }
                let mut commands = HashMap::new();
                commands.insert(
                    "this".to_string(),
                    CommandDefinition::new(
                        vec![
                            Parameter::new("a".into(), ExpressionShape::Integer),
                            Parameter::new("b".into(), ExpressionShape::Any),
                        ],
                        None,
                    ),
                );

                let mut scope = Box::new(Scope {
                    parent: None,
                    commands,
                });

                println!("{:#?}", parse(&input, 0, &mut scope));
                input.clear();
            }
            Err(error) => {
                println!("error: {}", error);
                break;
            }
        }
    }
    //let input = "this 3 a\nis { another; \npipeline } ; { [y] foo bar } 3";
    //let input = "?+󁃬z i￳.　";
}
