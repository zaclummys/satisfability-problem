use crate::expression::Expression;

/*
satisfies (Var a) true
	a is true

satisfies (Var a) false
	a is false


satisfies (Not a) true
	a is false

satisfies (Not a) false
	a is true


satisfies (AND a b) true
	a is true, b is true	

satisfies (AND a b) false
	a is false
	b is false

satisfies (OR a b) true
	a is true
	b is true

satisfies (OR a b) false
	a is false, b is false

this is a new theory.
All means that both sides must be satisfied.
Any means that at least one side must be satisfied.
Binding means that a name must satisfy a value (v).
There is no such expression as "not".
Each name can only be satisfied to a single value. If "a" is satisfied by "x", "a" cannot be satified by other value than "x".

Rules:
R -> All | Any | Binding | (R)
All -> All (R R)
Any-> Any (R R)
Binding -> Name S Value
Name -> [a-z]
Value-> [a-z]

Idempotence:
All (R, R) = R
Any (R, R) = R

Commutativity:
All (R1, R2) = All (R2, R1)
Any (R1, R2) = Any (R2, R1)

Associativity:
All (R1, All (R2, R3)) = All (All (R1, R2), R3)
Any (R1, Any (R2, R3)) = Any (Any (R1, R2), R3)

Distributive:
All (R1, All (R2, R3)) = All(All (R1, R2), All (R1, R3))
All (R1, Any (R2, R3)) = Any (All (R1, R2), All (R1, R3))
Any (R1, All (R2, R3)) = All (Any (R1, R2), Any (R1, R3))
Any (R1, Any (R2, R3)) = Any (Any (R1, R2), Any (R1, R3))

Absorption:
All (R1, Any (R1, R2)) = Any (All (R1, R1), All (R1, R3)) = Any (R1, All (R1, R3)) = R1
Any (R1, All (R1, R3)) = All (Any (R1, R1), Any (R1, R3)) = All (R1, Any (R1, R3)) = R1

Contradiction:
All (a S x, a S y) = Never
All (a S x, All(R, a S y)) = Never

Null:
All (R, Never) = Never
All (Never, Never) = Never
Any (Never, Never) = Never

Identity:
Any (R, Contradiction) = R

Tautology:
Any (a S x1, a S x2) = Always, if x = {x1, x2}

All means that both sides must be satisfied.
Any means that at least one side must be satisfied.
Binding means that a name must satisfy a value (v).
There is no such expression as "not".
Each name can only be satisfied to a single value. If "a" is satisfied by "x", "a" cannot be satified by other value than "x".
 */

#[derive(Debug, Clone)]
pub enum Requirement {
    Var (String, bool),

    All (Box<Requirement>, Box<Requirement>),
    Any (Box<Requirement>, Box<Requirement>),

    Always,
    Never,
}

impl PartialEq for Requirement {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Always, Self::Always) => true,
            (Self::Never, Self::Never) => true,

            (Self::Var(ln, lv), Self::Var(rn, rv)) => ln == rn && lv == rv,
            (Self::All(ll, lr), Self::All(rl, rr)) => (ll == rl && lr == rr) || (ll == rr && lr == rl),
            (Self::Any(ll, lr), Self::Any(rl, rr)) => (ll == rl && lr == rr) || (ll == rr && lr == rl),

            _ => false
        }
    }
}

impl Requirement {
    #[inline]
    fn all (left: Requirement, right: Requirement) -> Requirement {
        match (left, right) {
            // Idempotence Law
            (left, right) if left == right => left,

            // Null Law
            (Requirement::Never, _) => Requirement::Never,
            (_, Requirement::Never) => Requirement::Never,

            // Contradiction Law
            (Requirement::Var(ln, lv), Requirement::Var(rn, rv)) if ln == rn && lv != rv => Requirement::Never,

            // Distributive Law on Right Side
            (left, Requirement::Any (left_of_any, right_of_any)) => {
                Requirement::any(
                    Requirement::all(left.clone(), *left_of_any),
                    Requirement::all(left.clone(), *right_of_any),
                )
            }
            
            // Distributive Law on Left Side
            (Requirement::Any (left_of_any, right_of_any), right) => {
                Requirement::any(
                    Requirement::all(*left_of_any, right.clone()),
                    Requirement::all(*right_of_any, right.clone()),
                )
            }

            (left, right) => Requirement::All(
                Box::new(left),
                Box::new(right),
            )
        }
    }

    #[inline]
    fn any (left: Requirement, right: Requirement) -> Requirement {
        match (left, right) {
            // Idempotence Law
            (left, right) if left == right => left,

            // Null Law
            (Requirement::Never, Requirement::Never) => Requirement::Never,

            // Identity Law
            (Requirement::Never, right) => right,
            (left, Requirement::Never) => left,

            // Tautology Law
            (Requirement::Var(ln, lv), Requirement::Var(rn, rv)) if ln == rn && lv != rv => Requirement::Always,

            (left, right) => Requirement::Any(
                Box::new(left),
                Box::new(right),
            )
        }
    }

    pub fn format (self) {
        match self {
            Requirement::Any (left, right) => {
                left.format();

                // println!();
                println!();

                right.format();
                
                // println!();
            }

            Requirement::All (left, right) => {
                left.format();

                print!(", ");

                right.format()
            }

            Requirement::Var(name, value) => {
                print!("{} -> {}", name, value);
            }

            req => unimplemented!("{:?}", req),
        }
    }
}

pub struct DynamicSatisfability<'a> {
    took: usize,
    expression: &'a Expression,
}

impl<'a> DynamicSatisfability<'a> {
    pub fn new (expression: &'a Expression) -> DynamicSatisfability<'a> {
        DynamicSatisfability {
            expression,
            took: 0,
        }
    }

    fn satisfies_expression (&self, expression: &Expression, expectative: bool) -> Requirement {        
        match expression {
            Expression::Var (name) => Requirement::Var(name.clone(), expectative),

            Expression::Not (inner) => {
                self.satisfies_expression(inner, !expectative)
            }

            Expression::Or(left, right) => {
                let left_requirement = self.satisfies_expression(left, expectative);
                let right_requirement = self.satisfies_expression(right, expectative);

                if expectative {
                    Requirement::any(
                        left_requirement,
                        right_requirement,
                    )
                } else {
                    Requirement::all(
                        left_requirement,
                        right_requirement,
                    )
                }
            }

            Expression::And(left, right) => {
                let left_requirement = self.satisfies_expression(left, expectative);
                let right_requirement = self.satisfies_expression(right, expectative);

                if expectative {
                    Requirement::all(
                        left_requirement,
                        right_requirement,
                    )
                } else {
                    Requirement::any(
                        left_requirement,
                        right_requirement,
                    )
                }
            }

            Expression::Xor (left, right) => {
                let left_true = self.satisfies_expression(left, true);
                let left_false = self.satisfies_expression(left, false);
                
                let right_true = self.satisfies_expression(right, true);
                let right_false = self.satisfies_expression(right, false);

                if expectative {
                    Requirement::any(
                        Requirement::all(left_true, right_false),
                        Requirement::all(left_false, right_true),
                    )
                } else {
                    Requirement::any(
                        Requirement::all(left_true, right_true),
                        Requirement::all(left_false, right_false),
                    )
                }                            
            }

            Expression::False => match expectative {
                true => Requirement::Never,
                false => Requirement::Always,
            }

            Expression::True => match expectative {
                true => Requirement::Always,
                false => Requirement::Never,
            }

        }
    }

    pub fn satisfies (&self, expectative: bool) -> Requirement {
        self.satisfies_expression(self.expression, expectative)
    }

    pub fn took (&self) -> usize {
        self.took
    }
}

#[cfg(test)]
mod test {
    // use crate::expression;

    use super::*;

    #[test]
    fn var_expression_should_satisfies_true () {
        let expression = Expression::Var("a".to_string());

        let mut satisfability = DynamicSatisfability::new(&expression);
        
        assert_eq!(satisfability.satisfies(true), Requirement::Var("a".to_string(), true));
    }

    #[test]
    fn var_expression_should_satisfies_false () {
        let expression = Expression::Var("a".to_string());

        let mut satisfability = DynamicSatisfability::new(&expression);
        
        assert_eq!(satisfability.satisfies(false), Requirement::Var("a".to_string(), false));
    }

    #[test]
    fn not_expression_should_satisfies_true () {
        let expression = Expression::Not(
            Box::new(
                Expression::Var("a".to_string())
            )
        );

        let mut satisfability = DynamicSatisfability::new(&expression);
        
        assert_eq!(satisfability.satisfies(true), Requirement::Var("a".to_string(), false));
    }

    #[test]
    fn not_expression_should_satisfies_false () {
        let expression = Expression::Not(
            Box::new(
                Expression::Var("a".to_string())
            )
        );

        let mut satisfability = DynamicSatisfability::new(&expression);
        
        assert_eq!(satisfability.satisfies(false), Requirement::Var("a".to_string(), true));
    }

    #[test]
    fn and_expression_should_satisfies_true () {
        let expression = Expression::And(
            Box::new(
                Expression::Var("a".to_string())
            ),
            Box::new(
                Expression::Var("b".to_string())
            ),
        );

        let mut satisfability = DynamicSatisfability::new(&expression);
        
        assert_eq!(
            satisfability.satisfies(true),
            
            Requirement::All(
                Box::new(Requirement::Var("a".to_string(), true)),
                Box::new(Requirement::Var("b".to_string(), true)),
            )
        );
    }

    #[test]
    fn and_expression_should_satisfies_false () {
        let expression = Expression::And(
            Box::new(
                Expression::Var("a".to_string())
            ),
            Box::new(
                Expression::Var("b".to_string())
            ),
        );

        let mut satisfability = DynamicSatisfability::new(&expression);
        
        assert_eq!(
            satisfability.satisfies(false),
            
            Requirement::Any(
                Box::new(Requirement::Var("a".to_string(), false)),
                Box::new(Requirement::Var("b".to_string(), false)),
            )
        );
    }

    #[test]
    fn or_expression_should_satisfies_true () {
        let expression = Expression::Or(
            Box::new(
                Expression::Var("a".to_string())
            ),
            Box::new(
                Expression::Var("b".to_string())
            ),
        );

        let mut satisfability = DynamicSatisfability::new(&expression);
        
        assert_eq!(
            satisfability.satisfies(true),

            Requirement::Any(
                Box::new(Requirement::Var("a".to_string(), true)),
                Box::new(Requirement::Var("b".to_string(), true)),
            )
        );  
    }

    #[test]
    fn or_expression_should_satisfies_false () {
        let expression = Expression::Or(
            Box::new(
                Expression::Var("a".to_string())
            ),
            Box::new(
                Expression::Var("b".to_string())
            ),
        );

        let mut satisfability = DynamicSatisfability::new(&expression);

        assert_eq!(
            satisfability.satisfies(false),

            Requirement::All(
                Box::new(Requirement::Var("a".to_string(), false)),
                Box::new(Requirement::Var("b".to_string(), false)),
            )
        );
    }

    #[test]
    fn not_and_expression_should_satisfies_true () {
        let expression = Expression::Not(
            Box::new(
                Expression::And(
                    Box::new(
                        Expression::Var("a".to_string())
                    ),
                    Box::new(
                        Expression::Var("b".to_string())
                    ),
                )
            )
        );

        let mut satisfability = DynamicSatisfability::new(&expression);
        
        assert_eq!(
            satisfability.satisfies(true),

            Requirement::Any(
                Box::new(Requirement::Var("a".to_string(), false)),
                Box::new(Requirement::Var("b".to_string(), false)),
            )
        );
    }

    #[test]
    fn not_and_expression_should_satisfies_false () {
        let expression = Expression::Not(
            Box::new(
                Expression::And(
                    Box::new(
                        Expression::Var("a".to_string())
                    ),
                    Box::new(
                        Expression::Var("b".to_string())
                    ),
                )
            )
        );

        let mut satisfability = DynamicSatisfability::new(&expression);
        
        assert_eq!(
            satisfability.satisfies(false),
            
            Requirement::All(
                Box::new(Requirement::Var("a".to_string(), true)),
                Box::new(Requirement::Var("b".to_string(), true)),
            )
        );
    }
}