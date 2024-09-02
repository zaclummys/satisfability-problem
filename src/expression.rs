#[derive(Debug, Clone)]
pub enum Expression<T> {
    Var (T),

    Not (Box<Expression<T>>),
    Or (Box<Expression<T>>, Box<Expression<T>>),
    And (Box<Expression<T>>, Box<Expression<T>>),

    Xor (Box<Expression<T>>, Box<Expression<T>>),

    True,
    False,
}

impl<T: PartialEq> PartialEq for Expression<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Not(a), Self::Not(b)) => a == b,

            (
                Self::Or(left_left, left_right),
                Self::Or(right_left, right_right)
            ) => (left_left == right_left && left_right == right_right) || (left_left == right_right && left_right == right_left),

            (
                Self::And(left_left, left_right),
                Self::And(right_left, right_right)
            ) => (left_left == right_left && left_right == right_right) || (left_left == right_right && left_right == right_left),

            (Self::Var(a), Self::Var(b)) => a == b,

            _ => false,
        }
    }
}

impl<T: Clone + PartialEq + std::fmt::Debug> Expression<T> {
    pub fn from_expressions<I: IntoIterator<Item = Expression<T>>> (expressions: I) -> Option<Expression<T>> {
        expressions.into_iter().reduce(|left, right| {
            Expression::And(Box::new(left), Box::new(right))
        })
    }

    pub fn de_morgan (self) -> Expression<T> {
        match self {
            Expression::Not (a) => match a.de_morgan() {
                Expression::And (left, right) => {
                    Expression::Or(
                        Box::new(Expression::Not(left)),
                        Box::new(Expression::Not(right)),
                    )
                }

                Expression::Or (left, right) => {
                    Expression::And(
                        Box::new(Expression::Not(left)),
                        Box::new(Expression::Not(right)),
                    )
                }

                expression => Expression::Not(Box::new(expression))
            }

            expression => expression
        }
    }

    /**
     * Transform the expression into a optimized version.
     * Cannot introduce more expressions than there was previously.
     */
    pub fn optimize (self) -> Expression<T> {
        println!("Optimizing expression");

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
                    (_, Expression::False) => Expression::False,
                    (Expression::False, _) => Expression::False,

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
                Expression::True => Expression::False,
                Expression::False => Expression::True,

                Expression::Not (b) => *b,

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
                    (_, Expression::True) => Expression::True,
                    (Expression::True, _) => Expression::True,

                    // Complement Law
                    (left, Expression::Not (right)) if left == *right => Expression::True,
                    (Expression::Not (left), right) if *left == right => Expression::True,

                    // Absortion Law
                    (Expression::And (left_left, left_right), right) if *left_left == right || *left_right == right => right,
                    (left, Expression::And (right_left, right_right)) if *right_left == left || *right_right == left => left,
                    
                    (left, right) => Expression::Or(
                        Box::new(left),
                        Box::new(right),
                    )
                }
            }

            Expression::Xor (left, right) => {
                let left = left.optimize();
                let right = right.optimize();

                Expression::Xor(
                    Box::new(left),
                    Box::new(right),
                )
            }

            expression => expression,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_be_equal () {
        let comparisons = [
            (Expression::True, Expression::True),
            (Expression::False, Expression::False),

            (
                Expression::Var('a'),
                Expression::Var('a'),
            ),

            (
                Expression::Not(Box::new(Expression::Var('a'))),
                Expression::Not(Box::new(Expression::Var('a'))),
            ),

            (
                Expression::And(
                    Box::new(Expression::Var('a')),
                    Box::new(Expression::Var('a')),
                ),    
                Expression::And(
                    Box::new(Expression::Var('a')),
                    Box::new(Expression::Var('a')),
                )
            ),

            (
                Expression::And(
                    Box::new(Expression::Var('a')),
                    Box::new(Expression::Var('b')),
                ),    
                Expression::And(
                    Box::new(Expression::Var('a')),
                    Box::new(Expression::Var('b')),
                )
            ),

            (
                Expression::And(
                    Box::new(Expression::Var('a')),
                    Box::new(Expression::Var('b')),
                ),    
                Expression::And(
                    Box::new(Expression::Var('b')),
                    Box::new(Expression::Var('a')),
                )
            ),

            (
                Expression::Or(
                    Box::new(Expression::Var('a')),
                    Box::new(Expression::Var('b')),
                ),    
                Expression::Or(
                    Box::new(Expression::Var('a')),
                    Box::new(Expression::Var('b')),
                )
            ),

            (
                Expression::Or(
                    Box::new(Expression::Var('a')),
                    Box::new(Expression::Var('b')),
                ),    
                Expression::Or(
                    Box::new(Expression::Var('b')),
                    Box::new(Expression::Var('a')),
                )
            ),

            (
                Expression::Or(
                    Box::new(Expression::Var('a')),
                    Box::new(Expression::Var('a')),
                ),    
                Expression::Or(
                    Box::new(Expression::Var('a')),
                    Box::new(Expression::Var('a')),
                )
            ),
        ];

        for (left, right) in comparisons {
            assert_eq!(left, right);
            assert_eq!(right, left);
        }
    }

    #[test]
    fn should_not_be_equal () {
        let comparisons = [
            (Expression::True, Expression::False),
            (Expression::False, Expression::True),

            (
                Expression::Var('a'),
                Expression::Var('b'),
            ),

            (
                Expression::Not(Box::new(Expression::Var('a'))),
                Expression::Not(Box::new(Expression::Var('b'))),
            ),

            (
                Expression::And(
                    Box::new(Expression::Var('a')),
                    Box::new(Expression::Var('a')),
                ),    
                Expression::And(
                    Box::new(Expression::Var('b')),
                    Box::new(Expression::Var('b')),
                )
            ),

            (
                Expression::And(
                    Box::new(Expression::Var('a')),
                    Box::new(Expression::Var('a')),
                ),    
                Expression::And(
                    Box::new(Expression::Var('b')),
                    Box::new(Expression::Var('c')),
                )
            ),

            (
                Expression::And(
                    Box::new(Expression::Var('a')),
                    Box::new(Expression::Var('b')),
                ),    
                Expression::And(
                    Box::new(Expression::Var('c')),
                    Box::new(Expression::Var('c')),
                ),
            ),

            (
                Expression::And(
                    Box::new(Expression::Var('a')),
                    Box::new(Expression::Var('b')),
                ),    
                Expression::And(
                    Box::new(Expression::Var('c')),
                    Box::new(Expression::Var('d')),
                )
            ),

            (
                Expression::Or(
                    Box::new(Expression::Var('a')),
                    Box::new(Expression::Var('a')),
                ),    
                Expression::Or(
                    Box::new(Expression::Var('b')),
                    Box::new(Expression::Var('b')),
                )
            ),

            (
                Expression::Or(
                    Box::new(Expression::Var('a')),
                    Box::new(Expression::Var('a')),
                ),    
                Expression::Or(
                    Box::new(Expression::Var('b')),
                    Box::new(Expression::Var('c')),
                )
            ),

            (
                Expression::Or(
                    Box::new(Expression::Var('a')),
                    Box::new(Expression::Var('b')),
                ),    
                Expression::Or(
                    Box::new(Expression::Var('c')),
                    Box::new(Expression::Var('c')),
                ),
            ),

            (
                Expression::Or(
                    Box::new(Expression::Var('a')),
                    Box::new(Expression::Var('b')),
                ),    
                Expression::Or(
                    Box::new(Expression::Var('c')),
                    Box::new(Expression::Var('d')),
                )
            ),
        ];

        for (left, right) in comparisons {
            assert_ne!(left, right);
            assert_ne!(right, left);
        }
    }

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
    fn should_optimize_not_true () {
        let expression: Expression<()> = Expression::Not(
            Box::new(
                Expression::True
            )
        );
        
        assert_eq!(expression.optimize(), Expression::False);
    }

    #[test]
    fn should_optimize_not_false () {
        let expression: Expression<()> = Expression::Not(
            Box::new(
                Expression::False
            )
        );
        
        assert_eq!(expression.optimize(), Expression::True);
    }

    #[test]
    fn should_apply_de_morgan_law_to_not_and () {
        let expression = Expression::Not(
            Box::new(
                Expression::And(
                    Box::new(
                        Expression::Var('a')
                    ),
                    Box::new(
                        Expression::Var('b')
                    )
                )
            )
        );
        
        assert_eq!(
            expression.de_morgan(),

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
        );
    }

    #[test]
    fn should_apply_de_morgan_law_to_not_or () {
        let expression = Expression::Not(
            Box::new(
                Expression::Or(
                    Box::new(
                        Expression::Var('a')
                    ),
                    Box::new(
                        Expression::Var('b')
                    )
                )
            )
        );
        
        assert_eq!(
            expression.de_morgan(),

            Expression::And(
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
        );
    }

    #[test]
    fn should_not_apply_de_morgan_law_when_to_not_var () {
        let expression = Expression::Not(
            Box::new(
                Expression::Var('a')
            )
        );
        
        assert_eq!(
            expression.de_morgan(),

            Expression::Not(
                Box::new(
                    Expression::Var('a')
                )
            )
        );
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
    fn should_optimize_and_with_identity_law () {
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
    fn should_optimize_and_with_null_law () {
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
    fn should_optimize_and_with_complement_law () {
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
    fn should_optimize_or_with_complement_law () {
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
    fn should_optimize_or_with_absortion_law () {
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
    #[ignore]
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
    #[ignore]
    fn should_not_optimize_or_with_anti_distributive_law_when_the_two_inner_expressions_are_and_but_have_different_terms () {
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
    #[ignore]
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

    #[test]
    #[ignore]
    fn should_optimize_to_common_scenarios () {
        let scenarios = [
            // Distributive -> Complement -> Identity
            (
                Expression::And(
                    Box::new(
                        Expression::Var('a')
                    ),
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
                                Expression::Var('b')
                            ),
                        )
                    ),
                ),

                Expression::And(
                    Box::new(
                        Expression::Var('a')
                    ),
                    Box::new(
                        Expression::Var('b')
                    )
                ),
            ),

        ];

        for (before, after) in scenarios {
            assert_eq!(before.optimize(), after);
        }
    }
}