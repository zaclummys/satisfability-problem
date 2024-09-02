mod expression;
mod parser;
mod lexer;
mod token;
mod satisfability;

use satisfability::DynamicSatisfability;

use self::expression::{
    Expression,
};

fn main () {  
    let expression = (1..5).into_iter().fold(Expression::Var('0'), |a, b| Expression::Xor(
        Box::new(
            a
        ),
        Box::new(
            Expression::Var(std::char::from_u32(b).unwrap()),
        )
    ))
        .de_morgan()
        .optimize()
        .simplify();

    let satisfability = DynamicSatisfability::new(&expression);

    println!("{:#?}", satisfability.satisfies(true));
}
