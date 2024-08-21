#[derive(Eq, PartialEq, Debug, Clone)]
enum Expression {
    Not (Box<Expression>),
    Or (Box<Expression>, Box<Expression>),
    And (Box<Expression>, Box<Expression>),
    Var (char),

    True,
    False,
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
                    // Idempotent Law
                    (left, right) if left == right => left,

                    // Identity Law
                    (left, Expression::True) => left,
                    (Expression::True, right) => right,

                    // Null Law
                    (left, Expression::False) => Expression::False,
                    (Expression::False, right) => Expression::False,

                    // Complement Law
                    (left, Expression::Not (right)) if left == *right => Expression::False,
                    (Expression::Not (left), right) if *left == right => Expression::False,
                    
                    // Absorption Law
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
                    // Idempotent Law
                    (left, right) if left == right => left,

                    // Identity Law
                    (left, Expression::False) => left,
                    (Expression::False, right) => right,

                    // Null Law
                    (left, Expression::True) => Expression::True,
                    (Expression::True, right) => Expression::True,

                    // Complement Law
                    (left, Expression::Not (right)) if left == *right => Expression::True,
                    (Expression::Not (left), right) if *left == right => Expression::True,

                    // Anti-distributive Law                    
                    (
                        Expression::And (left_left, left_right),
                        Expression::And (right_left, right_right),
                    ) => {
                        if *left_left == *right_left => {
                            Expression::And(
                                left_left,
                                Box::new(
                                    Expression::Or(
                                        left_right,
                                        right_right,
                                    )
                                )
                            ).optimize()
                        }
                    }

                    (
                        Expression::And (left_left, left_right),
                        Expression::And (right_left, right_right),
                    ) if *left_left == *right_right => {
                        Expression::And(
                            left_left,
                            Box::new(
                                Expression::Or(
                                    left_right,
                                    right_left,
                                )
                            )
                        ).optimize()
                    }

                    (
                        Expression::And (left_left, left_right),
                        Expression::And (right_left, right_right),
                    ) if *left_right == *right_left => {
                        Expression::And(
                            left_right,
                            Box::new(
                                Expression::Or(
                                    left_left,
                                    right_right,
                                )
                            )
                        ).optimize()
                    }

                    (
                        Expression::And (left_left, left_right),
                        Expression::And (right_left, right_right),
                    ) if *left_right == *right_right => {
                        Expression::And(
                            left_right,
                            Box::new(
                                Expression::Or(
                                    left_left,
                                    right_left,
                                )
                            )
                        ).optimize()
                    }

                    // Absortion Law
                    (Expression::And (left_left, left_right), right) if *left_left == right || *left_right == right => right,
                    (left, Expression::And (right_left, right_right)) if *right_left == left || *right_right == left => left,
                    
                    (left, right) => Expression::Or(
                        Box::new(left),
                        Box::new(right),
                    )
                }
            }

            expression => expression,
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
    
    // (A∧B)∨(¬A∧B)∨(A∧¬B)

    let expression1 = Expression::Or(
        Box::new(
            Expression::And(
                Box::new(
                    Expression::Var('a')
                ),
                Box::new(
                    Expression::Var('b')
                ),
            )
        ),
        Box::new(
            Expression::And(
                Box::new(
                    Expression::Not(
                        Box::new(
                            Expression::Var('a')
                        )
                    )
                ),
                Box::new(
                    Expression::Var('b')
                ),
            )
        ),
    );
    
    println!("{:#?}", expression1);
    println!();
    println!("{:#?}", expression1.clone().optimize());
    println!("{:#?}", expression1.clone().optimize().optimize());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_optimize_not () {
        let expression = Expression::Not(
            Box::new(
                Expression::Var('a')
            )
        );
        
        assert_eq!(expression.optimize(), Expression::Not(
            Box::new(
                Expression::Var('a')
            )
        ));
    }
    
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
    fn should_optimize_and_with_idempotent_law () {
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
    fn should_optimize_and_expression_with_identity_law () {
        let expressions = [
            // Left
            Expression::And(
                Box::new(
                    Expression::True
                ),
                Box::new(
                    Expression::Var('a'),
                ),
            ),

            // Right
            Expression::And(
                Box::new(
                    Expression::Var('a'),
                ),
                Box::new(
                    Expression::True
                ),
            ),
        ];
        
        for expression in expressions {
            assert_eq!(expression.optimize(), Expression::Var('a'));
        }
    }

    #[test]
    fn should_optimize_and_expression_with_null_law () {
        let expressions = [
            // Left
            Expression::And(
                Box::new(
                    Expression::False
                ),
                Box::new(
                    Expression::Var('a'),
                ),
            ),

            // Right
            Expression::And(
                Box::new(
                    Expression::Var('a'),
                ),
                Box::new(
                    Expression::False
                ),
            ),
        ];
        
        for expression in expressions {
            assert_eq!(expression.optimize(), Expression::False);
        }
    }

    #[test]
    fn should_optimize_and_expression_with_complement_law () {
        let expressions = [
            // Left
            Expression::And(
                Box::new(
                    Expression::Not(
                        Box::new(
                            Expression::Var('a'),
                        ),
                    )
                ),
                Box::new(
                    Expression::Var('a'),
                ),
            ),

            // Right
            Expression::And(
                Box::new(
                    Expression::Var('a'),
                ),
                Box::new(
                    Expression::Not(
                        Box::new(
                            Expression::Var('a'),
                        ),
                    )
                ),
            ),
        ];
        
        for expression in expressions {
            assert_eq!(expression.optimize(), Expression::False);
        }
    }
    
    #[test]
    fn should_optimize_and_with_absortion_law () {
        let expressions = [
            // Left left
            Expression::And(
                Box::new(
                    Expression::Or(
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
            ),

            // Left right
            Expression::And(
                Box::new(
                    Expression::Or(
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
            ),

            // Right left
            Expression::And(
                Box::new(
                    Expression::Var('a'),
                ),
                Box::new(
                    Expression::Or(
                        Box::new(
                            Expression::Var('a'),
                        ),
                        Box::new(
                            Expression::Var('b'),
                        ),
                    )
                ),
            ),

            // Right right
            Expression::And(
                Box::new(
                    Expression::Var('a'),
                ),
                Box::new(
                    Expression::Or(
                        Box::new(
                            Expression::Var('b'),
                        ),
                        Box::new(
                            Expression::Var('a'),
                        ),
                    )
                ),
            ),
        ];

        for expression in expressions {
            assert_eq!(expression.optimize(), Expression::Var('a'));
        }
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
    fn should_optimize_or_with_idempotent_law () {
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
    fn should_optimize_or_with_identity_law () {
        let expressions = [
            // Left
            Expression::Or(
                Box::new(
                    Expression::False,
                ),
                Box::new(
                    Expression::Var('a')
                ),
            ),

            // Right
            Expression::Or(
                Box::new(
                    Expression::Var('a')
                ),
                Box::new(
                    Expression::False,
                ),
            )
        ];

        for expression in expressions {
            assert_eq!(expression.optimize(), Expression::Var('a'));
        }
    }

    #[test]
    fn should_optimize_or_with_null_law () {
        let expressions = [
            // Left
            Expression::Or(
                Box::new(
                    Expression::True,
                ),
                Box::new(
                    Expression::Var('a')
                ),
            ),

            // Right
            Expression::Or(
                Box::new(
                    Expression::Var('a')
                ),
                Box::new(
                    Expression::True,
                ),
            )
        ];

        for expression in expressions {
            assert_eq!(expression.optimize(), Expression::True);
        }
    }

    #[test]
    fn should_optimize_or_expression_with_complement_law () {
        let expressions = [
            // Left
            Expression::Or(
                Box::new(
                    Expression::Not(
                        Box::new(
                            Expression::Var('a'),
                        ),
                    )
                ),
                Box::new(
                    Expression::Var('a'),
                ),
            ),

            // Right
            Expression::Or(
                Box::new(
                    Expression::Var('a'),
                ),
                Box::new(
                    Expression::Not(
                        Box::new(
                            Expression::Var('a'),
                        ),
                    )
                ),
            ),
        ];
        
        for expression in expressions {
            assert_eq!(expression.optimize(), Expression::True);
        }
    }
    
    #[test]
    fn should_optimize_or_expression_with_absortion_law () {
        let expressions = [
            // Left left
            Expression::Or(
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
            ),

            // Left right
            Expression::Or(
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
            ),
            
            // Right left
            Expression::Or(
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
            ),
                
            // Right right
            Expression::Or(
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
            ),
        ];
        
        for expression in expressions {
            assert_eq!(expression.optimize(), Expression::Var('a'));
        }
    }

    #[test]
    fn should_optimize_or_with_anti_distributive_law () {
        let expressions = [
            // Left left, Right left
            Expression::Or(
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
                    Expression::And(
                        Box::new(
                            Expression::Var('a'),
                        ),
                        Box::new(
                            Expression::Var('c'),
                        ),
                    )
                ),
            ),

            // Left left, Righ right
            Expression::Or(
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
                    Expression::And(
                        Box::new(
                            Expression::Var('c'),
                        ),
                        Box::new(
                            Expression::Var('a'),
                        ),
                    )
                ),
            ),

            // Left right, Right left
            Expression::Or(
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
                    Expression::And(
                        Box::new(
                            Expression::Var('a'),
                        ),
                        Box::new(
                            Expression::Var('c'),
                        ),
                    )
                ),
            ),

            // Left right, Right Right
            Expression::Or(
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
                    Expression::And(
                        Box::new(
                            Expression::Var('c'),
                        ),
                        Box::new(
                            Expression::Var('a'),
                        ),
                    )
                ),
            )
        ];

        for expression in expressions {
            assert_eq!(
                expression.optimize(),
    
                Expression::And(
                    Box::new(
                        Expression::Var('a')
                    ),
                    Box::new(
                        Expression::Or(
                            Box::new(
                                Expression::Var('b')
                            ),
                            Box::new(
                                Expression::Var('c')
                            ),
                        )
                    ),
                )
            );
        }
    }

    #[test]
    fn should_not_optimize_or_expression_with_anti_distributive_law_when_the_two_inner_expressions_are_and_but_have_different_terms () {
        let expression = Expression::Or(
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
                Expression::And(
                    Box::new(
                        Expression::Var('c'),
                    ),
                    Box::new(
                        Expression::Var('d'),
                    ),
                )
            ),
        );

        assert_eq!(expression.optimize(), Expression::Or(
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
                Expression::And(
                    Box::new(
                        Expression::Var('c'),
                    ),
                    Box::new(
                        Expression::Var('d'),
                    ),
                )
            ),
        ));
    }

    
    #[test]
    fn should_simplify_and () {
        let expression = Expression::And(
            Box::new(
                Expression::Var('a')
            ),
            Box::new(
                Expression::Var('b')
            ),
        );

        assert_eq!(expression.simplify(), Expression::Not(
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
                )
            )
        ));
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
    fn should_simplify_or () {
        let expression = Expression::Or(
            Box::new(
                Expression::Var('a')
            ),
            Box::new(
                Expression::Var('b')
            )
        );

        assert_eq!(expression.simplify(), Expression::Or(
            Box::new(
                Expression::Var('a')
            ),
            Box::new(
                Expression::Var('b')
            )
        ));
    }
    
    #[test]
    fn should_apply_de_morgan_law_to_and () {
        let expression = Expression::Not(
            Box::new(
                Expression::And(
                    Box::new(
                        Expression::Var('a')
                    ),
                    Box::new(
                        Expression::Var('b')
                    ),
                )
            )
        );
        
        assert_eq!(
            expression
                .optimize() 
                .simplify()
                .optimize(),

            Expression::Or(
                Box::new(
                    Expression::Not(
                        Box::new(
                            Expression::Var('a'),
                        )
                    )
                ),
                Box::new(
                    Expression::Not(
                        Box::new(
                            Expression::Var('b'),
                        )
                    )
                ),
            )
        );
    }

    #[test]
    fn should_not_apply_de_morgan_law_to_or () {
        let expression = Expression::Not(
            Box::new(
                Expression::Or(
                    Box::new(
                        Expression::Var('a')
                    ),
                    Box::new(
                        Expression::Var('b')
                    ),
                )
            )
        );
        
        assert_eq!(
            expression
                .optimize() 
                .simplify()
                .optimize(),

            Expression::Not(
                Box::new(
                    Expression::Or(
                        Box::new(
                            Expression::Var('a')
                        ),
                        Box::new(
                            Expression::Var('b')
                        ),
                    )
                )
            )
        );
    }

    #[test]
    fn should_apply_anti_distributive_and_complement_laws_in_a_single_optimization_pass () {
        let expression = Expression::Or(
            Box::new(
                Expression::And(
                    Box::new(
                        Expression::Var('a')
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
                        Expression::Not(
                            Box::new(
                                Expression::Var('b')
                            )
                        )
                    ),
                )
            ),
        );

        assert_eq!(expression.optimize(), Expression::Var('a'));
    }
}