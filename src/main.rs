mod expression;
mod parser;
mod lexer;
mod token;

use self::expression::{
    Expression,
    Satisfability,
    Expectative,
};

fn main () {  
    let expression = Expression::from_expressions([
        Expression::Not(
            Box::new(
                Expression::Var('x')
            )
        ),
        Expression::And(
            Box::new(
                Expression::Var('a')
            ),
            Box::new(
                Expression::Var('b')
            ),
        ),
        Expression::Or(
            Box::new(
                Expression::And(
                    Box::new(
                        Expression::Var('x')
                    ),
                    Box::new(
                        Expression::Var('b')
                    ),
                )
            ),
            Box::new(
                Expression::And(
                    Box::new(
                        Expression::Var('a')
                    ),
                    Box::new(
                        Expression::Var('y')
                    ),
                )
            ),
        )
    ])
        .unwrap()
        .optimize();


    let mut satisfability = Satisfability::new();

    let satisfies = satisfability.satisfies(&expression, Expectative::True);

    println!("Satisfies? {}", satisfies);
    println!("{:?}", satisfability.expectatives);

    // println!("{:#?}", expression);
    // println!();
    println!("{:#?}", expression
        .de_morgan()
        .optimize()
        .simplify()
        .optimize()
    );
}
