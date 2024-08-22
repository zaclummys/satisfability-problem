mod expression;

use self::expression::Expression;

fn main () {  
    let expression1 = Expression::from_expressions([
        Expression::Not(
            Box::new(
                Expression::Var('a')
            )
        ),
        Expression::And(
            Box::new(
                Expression::Not(
                    Box::new(
                        Expression::Var('b')
                    )
                )
            ),
            Box::new(
                Expression::Not(
                    Box::new(
                        Expression::Var('b')
                    )
                )
            )
        )
    ]).expect("No expression");

    println!("{:#?}", expression1);
    println!();
    println!("{:#?}", expression1.clone().optimize().simplify().optimize());
}
