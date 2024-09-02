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

R -> X | Any | All | (R)
Any -> R
All -> R

Distributive
All a (Any b c) => Any (All a b) (All a c)
All (a b) (Any c d) => Any (All (a b) c) (All (a b) d)

All x (Any a b) (Any c d) => Any (All x a c) (All x a d) (All x b c) (All x b d)

x and ((a or b) and (c or d))

All (Any a b) (Any c d) => Any (All a c) (All a d) (All b c) (All b d)
All (Any a b) (Any c d)

 */

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Requirement {
    Var (char, bool),
    All (Vec<Requirement>),
    Any (Vec<Requirement>),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Minirequirement {
    Var (char, bool),

    All (Box<Minirequirement>, Box<Minirequirement>),
    Any (Box<Minirequirement>, Box<Minirequirement>),
}

impl Minirequirement {
    fn distributive (self) -> Minirequirement {
        match self {
            Minirequirement::All (left, right) => {
                match (left.distributive(), right.distributive()) {
                    (
                        Minirequirement::Any(a, b),
                        Minirequirement::Any(x, y),
                    ) => {
                        let distributions = [
                            Minirequirement::All(a.clone(), x.clone()),
                            Minirequirement::All(a.clone(), y.clone()),
                            Minirequirement::All(b.clone(), x.clone()),
                            Minirequirement::All(b.clone(), y.clone()),
                        ];

                        distributions
                            .into_iter()
                            .reduce(|a, b| Minirequirement::Any(Box::new(a), Box::new(b)))
                            .unwrap()
                    }

                    (
                        a,
                        Minirequirement::Any(x, y),
                    ) => {
                        let distributions = [
                            Minirequirement::All(Box::new(a.clone()), x),
                            Minirequirement::All(Box::new(a.clone()), y),
                        ];

                        distributions
                            .into_iter()
                            .reduce(|a, b| Minirequirement::Any(Box::new(a), Box::new(b)))
                            .unwrap()
                    }
                    
                    (
                        Minirequirement::Any(a, b),
                        x,
                    ) => {
                        let distributions = [
                            Minirequirement::All(a, Box::new(x.clone())),
                            Minirequirement::All(b, Box::new(x.clone())),
                        ];

                        distributions
                            .into_iter()
                            .reduce(|a, b| Minirequirement::Any(Box::new(a), Box::new(b)))
                            .unwrap()
                    }

                    (left, right) => Minirequirement::All(
                        Box::new(left),
                        Box::new(right),
                    )
                }
            }

            minirequirement => minirequirement,
        }
    }
}


impl Requirement {
    fn all (left: Requirement, right: Requirement) -> Requirement {
        match (left, right) {
            (Requirement::Any (ls), Requirement::Any (rs)) => {
                let mut requirements = Vec::new();

                for l in ls.iter() {
                    for r in rs.iter() {
                        requirements.push(Requirement::all(
                            l.clone(),
                            r.clone(),
                        ));
                    }
                }

                Requirement::Any(requirements)
            }

            (l, Requirement::Any (rs)) => {
                let mut requirements = Vec::new();

                for r in rs {
                    requirements.push(Requirement::all(l.clone(), r));
                }

                Requirement::Any(requirements)
            }

            (Requirement::Any (ls), r) => {
                let mut requirements = Vec::new();

                for l in ls {
                    requirements.push(Requirement::all(l, r.clone()));
                }

                Requirement::Any(requirements)
            }

            (Requirement::All (ls), Requirement::All (rs)) => {
                Requirement::All(
                    ls.into_iter().chain(rs.into_iter()).collect()
                )
            }

            (Requirement::All (ls), r) => {
                Requirement::All(
                    ls.into_iter().chain(std::iter::once(r)).collect()
                )
            }

            (l, Requirement::All (rs)) => {
                Requirement::All(
                    std::iter::once(l).chain(rs.into_iter()).collect()
                )
            }

            (l, r) => Requirement::All(vec![l, r]),
        }
    }

    fn any (left: Requirement, right: Requirement) -> Requirement {
        match (left, right) {
            (Requirement::Any (ls), Requirement::Any (rs)) => {
                Requirement::Any(
                    ls.into_iter().chain(rs.into_iter()).collect()
                )
            }
            
            (Requirement::Any (ls), r) => {
                Requirement::Any(
                    ls.into_iter().chain(std::iter::once(r)).collect()
                )
            }

            (l, Requirement::Any (rs)) => {
                Requirement::Any(
                    std::iter::once(l).chain(rs.into_iter()).collect()
                )
            }

            (l, r) => Requirement::Any(vec![l, r]),
        }
    }
}

// #[derive(Default)]
// struct Requirements {
//     map: HashMap<char, bool>
// }

// impl Requirements {
//     fn with_requirement<I: Into<char>> (mut self, name: I, expectative: bool) -> Self {
//         self.map.insert(name.into(), expectative);

//         self
//     }

//     fn merge (self, other: Requirements) -> Option<Self> {
//         let mut new_map = HashMap::new();

//         for (other_name, other_expectative) in other.map.into_iter() {
//             match self.map.get(&other_name) {
//                 Some (expectative) if expectative != &other_expectative => {
//                     return None
//                 }

//                 _ => {
//                     new_map.insert(other_name, other_expectative);
//                 }
//             }
//         }

//         Some(Requirements {
//             map: new_map
//         })
//     }
// }

pub struct DynamicSatisfability<'a> {
    expression: &'a Expression,
}

impl<'a> DynamicSatisfability<'a> {
    pub fn new (expression: &'a Expression) -> DynamicSatisfability<'a> {
        DynamicSatisfability {
            expression
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

            _ => unimplemented!()
        }
    }

    pub fn satisfies (&self, expectative: bool) -> Requirement {
        self.satisfies_expression(self.expression, expectative)
    }
}

#[cfg(test)]
mod test {
    // use crate::expression;

    use super::*;

    #[test]
    fn var_expression_should_satisfies_true () {
        let expression = Expression::Var('a');

        let satisfability = DynamicSatisfability::new(&expression);
        
        assert_eq!(satisfability.satisfies(true), Requirement::Var('a', true));
    }

    #[test]
    fn var_expression_should_satisfies_false () {
        let expression = Expression::Var('a');

        let satisfability = DynamicSatisfability::new(&expression);
        
        assert_eq!(satisfability.satisfies(false), Requirement::Var('a', false));
    }

    #[test]
    fn not_expression_should_satisfies_true () {
        let expression = Expression::Not(
            Box::new(
                Expression::Var('a')
            )
        );

        let satisfability = DynamicSatisfability::new(&expression);
        
        assert_eq!(satisfability.satisfies(true), Requirement::Var('a', false));
    }

    #[test]
    fn not_expression_should_satisfies_false () {
        let expression = Expression::Not(
            Box::new(
                Expression::Var('a')
            )
        );

        let satisfability = DynamicSatisfability::new(&expression);
        
        assert_eq!(satisfability.satisfies(false), Requirement::Var('a', true));
    }

    #[test]
    fn and_expression_should_satisfies_true () {
        let expression = Expression::And(
            Box::new(
                Expression::Var('a')
            ),
            Box::new(
                Expression::Var('b')
            ),
        );

        let satisfability = DynamicSatisfability::new(&expression);
        
        assert_eq!(
            satisfability.satisfies(true),
            
            Requirement::All(vec![
                Requirement::Var('a', true),
                Requirement::Var('b', true),
            ])
        );
    }

    #[test]
    fn and_expression_should_satisfies_false () {
        let expression = Expression::And(
            Box::new(
                Expression::Var('a')
            ),
            Box::new(
                Expression::Var('b')
            ),
        );

        let satisfability = DynamicSatisfability::new(&expression);
        
        assert_eq!(satisfability.satisfies(false), Requirement::Any(vec![
            Requirement::Var('a', false),
            Requirement::Var('b', false),
        ]));
    }

    #[test]
    fn or_expression_should_satisfies_true () {
        let expression = Expression::Or(
            Box::new(
                Expression::Var('a')
            ),
            Box::new(
                Expression::Var('b')
            ),
        );

        let satisfability = DynamicSatisfability::new(&expression);
        
        assert_eq!(satisfability.satisfies(true), Requirement::Any(vec![
            Requirement::Var('a', true),
            Requirement::Var('b', true),
        ]));   
    }

    #[test]
    fn or_expression_should_satisfies_false () {
        let expression = Expression::Or(
            Box::new(
                Expression::Var('a')
            ),
            Box::new(
                Expression::Var('b')
            ),
        );

        let satisfability = DynamicSatisfability::new(&expression);

        assert_eq!(
            satisfability.satisfies(false),

            Requirement::All(vec![
                Requirement::Var('a', false),
                Requirement::Var('b', false),
            ])
        );
    }

    #[test]
    fn not_and_expression_should_satisfies_true () {
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

        let satisfability = DynamicSatisfability::new(&expression);
        
        assert_eq!(
            satisfability.satisfies(true),
            Requirement::Any(vec![
                Requirement::Var('a', false),
                Requirement::Var('b', false),
            ])
        );
    }

    #[test]
    fn not_and_expression_should_satisfies_false () {
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

        let satisfability = DynamicSatisfability::new(&expression);
        
        assert_eq!(satisfability.satisfies(false), Requirement::All(vec![
            Requirement::Var('a', true),
            Requirement::Var('b', true),
        ]));
    }

    #[test]
    fn or_expressions_should_merge () {
        let expression = Expression::Or(
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
        ).de_morgan().optimize();

        println!("{:#?}", expression);

        let satisfability = DynamicSatisfability::new(&expression);

        println!("{:#?}", satisfability.satisfies(true));
        
        assert_eq!(satisfability.satisfies(true), Requirement::Any(vec![
            Requirement::Var('a', true),
            Requirement::Var('b', true),
            Requirement::Var('c', true),
        ]));   
    }
}