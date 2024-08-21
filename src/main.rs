#[derive(Eq, PartialEq, Debug, Clone)]
enum Expression {
    Not (Box<Expression>),
    Or (Box<Expression>, Box<Expression>),
    And (Box<Expression>, Box<Expression>),
    Var (char),
}

impl Expression {
    fn from_expressions<I: IntoIterator<Item = Expression>> (expressions: I) -> Option<Expression> {
        expressions.into_iter().reduce(|left, right| {
            Expression::And(Box::new(left), Box::new(right))
        })
    }

    /**
     * Transform the expression into a optimized version.
     * Cannot introduce more expressions than there was previously.
     */
    fn optimize (self) -> Expression {
        match self {
            Expression::And (left, right) => {
                let left = left.optimize();
                let right = right.optimize();
                
                match (left, right) {
                    // (left, right) if left == right => left,
                    
                    (Expression::Or (left_left, left_right), right) if *left_left == right || *left_right == right => right,
                    (left, Expression::Or (right_left, right_right)) if *right_left == left || *right_right == left => left,
                    
                    (left, right) => Expression::And(
                        Box::new(left),
                        Box::new(right),
                    )
                }
            }
            
            Expression::Var (name) => Expression::Var(name),
            
            Expression::Not (a) => match a.optimize() {
              Expression::Not (b) => b.optimize(),
              a => Expression::Not(Box::new(a)),
            },
            
            Expression::Or (left, right) => {
                let left = left.optimize();
                let right = right.optimize();
                
                match (left, right) {
                    (left, right) if left == right => left,
                    
                    (Expression::And (left_left, left_right), right) if *left_left == right || *left_right == right => right,
                    (left, Expression::And (right_left, right_right)) if *right_left == left || *right_right == left => left,
                    
                    (left, right) => Expression::Or(
                        Box::new(left),
                        Box::new(right),
                    )
                }
            }
        }
    }
    
    /**
     * Transform the expression into a simplified version.
     * Can introduce more expressions than there was previously.
     */
    fn simplify (self) -> Expression {
        match self {
            Expression::And (left, right) => {
                let left = left.simplify();
                let right = right.simplify();
                
                Expression::Not(
                    Box::new(
                        Expression::Or(
                            Box::new(
                                Expression::Not(
                                    Box::new(left)
                                )
                            ),
                            Box::new(
                                Expression::Not(
                                    Box::new(right)
                                )
                            ),
                        )
                    )
                )
            }
            
            Expression::Not (inner) => Expression::Not(
                Box::new(inner.simplify())
            ),
            
            Expression::Or (left, right) => Expression::Or(
                Box::new(left.simplify()),
                Box::new(right.simplify()),
            ),
            
            expression => expression,
        }
    }
}

fn main () {
    use Expression::*;
    
    let expression1 = Expression::Not(
        Box::new(
            Expression::And(
                Box::new(
                    Expression::Var('p'),
                ),
                Box::new(
                    Expression::Not(
                        Box::new(
                            Expression::Not(
                                Box::new(
                                    Expression::Var('p')
                                )
                            )
                        )
                    )
                )
            )
        )
    );
    
    println!("{:#?}", expression1);
    println!();
    println!("{:#?}", expression1.clone().optimize().simplify());
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn should_optimize_double_not () {
        let double_not = Expression::Not(
            Box::new(
                Expression::Not(
                    Box::new(
                        Expression::Var('a')
                    )
                )
            )
        );
        
        assert_eq!(double_not.optimize(), Expression::Var('a'));
    }
    
    #[test]
    fn should_optimize_and () {
        let and = Expression::And(
            Box::new(
                Expression::Var('a'),
            ),
            Box::new(
                Expression::Var('b'),
            ),
        );
        
        assert_eq!(and.optimize(), Expression::And(
            Box::new(
                Expression::Var('a'),
            ),
            Box::new(
                Expression::Var('b'),
            ),
        ));
    }
    
    #[test]
    fn should_optimize_idempotent_and () {
        let idempotent_and = Expression::And(
            Box::new(
                Expression::Var('a'),
            ),
            Box::new(
                Expression::Var('a'),
            ),
        );
        
        assert_eq!(idempotent_and.optimize(), Expression::Var('a'));
    }
    
    #[test]
    fn should_optimize_or () {
        let or = Expression::Or(
            Box::new(
                Expression::Var('a'),
            ),
            Box::new(
                Expression::Var('b'),
            ),
        );
        
        assert_eq!(or.optimize(), Expression::Or(
            Box::new(
                Expression::Var('a'),
            ),
            Box::new(
                Expression::Var('b'),
            ),
        ));
    }
    
    #[test]
    fn should_optimize_idempotent_or () {
        let idempotent_or = Expression::Or(
            Box::new(
                Expression::Var('a'),
            ),
            Box::new(
                Expression::Var('a'),
            ),
        );
        
        assert_eq!(idempotent_or.optimize(), Expression::Var('a'));
    }
    
    #[test]
    fn should_optimize_or_expression_absorting_left_left () {
        let and_with_or_on_right = Expression::Or(
            Box::new(
                Expression::And(
                    Box::new(
                        Expression::Var('a'),
                    ),
                    Box::new(
                        Expression::Var('b'),
                    ),
                )
            ),
            Box::new(
                Expression::Var('a'),
            ),
        );
        
        assert_eq!(and_with_or_on_right.optimize(), Expression::Var('a'));
    }
    
    #[test]
    fn should_optimize_or_expression_absorting_left_right () {
        let and_with_or_on_right = Expression::Or(
            Box::new(
                Expression::And(
                    Box::new(
                        Expression::Var('b'),
                    ),
                    Box::new(
                        Expression::Var('a'),
                    ),
                )
            ),
            Box::new(
                Expression::Var('a'),
            ),
        );
        
        assert_eq!(and_with_or_on_right.optimize(), Expression::Var('a'));
    }
    
    #[test]
    fn should_optimize_or_expression_absorting_right_left () {
        let and_with_or_on_right = Expression::Or(
            Box::new(
                Expression::Var('a'),
            ),
            Box::new(
                Expression::And(
                    Box::new(
                        Expression::Var('a'),
                    ),
                    Box::new(
                        Expression::Var('b'),
                    ),
                )
            ),
        );
        
        assert_eq!(and_with_or_on_right.optimize(), Expression::Var('a'));
    }
    
    #[test]
    fn should_optimize_or_expression_absorting_right_right () {
        let and_with_or_on_right = Expression::Or(
            Box::new(
                Expression::Var('a'),
            ),
            Box::new(
                Expression::And(
                    Box::new(
                        Expression::Var('b'),
                    ),
                    Box::new(
                        Expression::Var('a'),
                    ),
                )
            ),
        );
        
        assert_eq!(and_with_or_on_right.optimize(), Expression::Var('a'));
    }
    
    #[test]
    fn should_simplify_not () {
        let not = Expression::Not(
            Box::new(
                Expression::Var('a')
            )
        );
        
        assert_eq!(not.simplify(), Expression::Not(
            Box::new(
                Expression::Var('a')
            )
        ));
    }
    
    #[test]
    fn should_simplify_not_recursively () {
        let not = Expression::Not(
            Box::new(
                Expression::And(
                    Box::new(
                        Expression::Var('a'),
                    ),
                    Box::new(
                        Expression::Var('b'),
                    )
                )
            )
        );
        
        assert_eq!(not.simplify(), Expression::Not(
            Box::new(
                Expression::Not(
                    Box::new(
                        Expression::Or(
                            Box::new(
                                Expression::Not(
                                    Box::new(
                                        Expression::Var('a')
                                    )
                                )
                            ),
                            Box::new(
                                Expression::Not(
                                    Box::new(
                                        Expression::Var('b')
                                    )
                                )
                            ),
                        ),
                    )
                )
            )
        ));
    }
    
    #[test]
    fn simplify_and_optimize_should_be_commutative () {
        let expression = Expression::Not(
            Box::new(
                Expression::Var('a')
            )
        );
        
        assert_eq!(
            expression.clone().simplify().optimize(),
            expression.clone().optimize().simplify(),
        )
    }
}