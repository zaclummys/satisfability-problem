mod expression;
mod parser;
mod lexer;
mod satisfability;
mod cli;

use cli::CLI;

use lexer::Lexer;
use parser::Parser;
use satisfability::DynamicSatisfability;

fn main () {  
    let arguments = CLI::arguments();

    let lexer = Lexer::new(arguments.string());

    let mut parser = Parser::new(lexer);

    let expression = match parser.parse() {
        Ok (expression) => expression,
        Err (error) => {
            panic!("{:?}", error);
        }
    };

    println!("Expression:");
    println!();
    println!("{:#?}", expression);
    println!();

    let expression = expression
        .de_morgan()
        .optimize();

    println!();

    println!("Optimized Expression:");
    println!();

    println!("{:#?}", expression);

    println!();

    let satisfability = DynamicSatisfability::new(&expression);

    for expectative in [true] {
        let requirements = satisfability.satisfies(expectative);
    
        println!();
        println!("Requirements to be {}:", expectative);
        println!();

        println!("{:#?}", requirements);

        requirements.format();

        println!();
    }
}
