mod expression;

use self::expression::Expression;

fn main () {  
    let expression = Expression::from_expressions([
        Expression::Or(
            Box::new(
                Expression::Var('a')
            ),
            Box::new(
                Expression::Var('b')
            ),
        ),
        
        Expression::Or(
            Box::new(
                Expression::Not(
                    Box::new(
                        Expression::Var('b')
                    )
                )
            ),
            Box::new(
                Expression::Var('c')
            ),
        )
    ]).expect("No expression");

    println!("{:#?}", expression);
    println!();
    println!("{:#?}", expression.clone().apply());
}
