use crate::expression::Expression;

pub enum Expectative {
    True,
    False,
}

pub struct GeneralSatisfability<'a> {
    expression: &'a Expression,
}

impl<'a> GeneralSatisfability<'a> {
    pub fn new (expression: &'a Expression) -> GeneralSatisfability<'a> {
        GeneralSatisfability {
            expression
        }
    }

    fn satisfies_expression (&self, expression: &Expression, expectative: Expectative) -> bool {
        match expression {
            Expression::Var (_) => true,

            Expression::True => match expectative {
                Expectative::True => true,
                Expectative::False => false,
            }

            Expression::False => match expectative {
                Expectative::False => true,
                Expectative::True => false,
            }

            Expression::Not (inner) => {
                self.satisfies_expression(inner, match expectative {
                    Expectative::True => Expectative::False,
                    Expectative::False => Expectative::True,
                })
            }

            Expression::Or(left, right) => {
                let left_true = self.satisfies_expression(left, Expectative::True);
                let right_true = self.satisfies_expression(right, Expectative::True);

                left_true || right_true
            }

            Expression::And(left, right) => {
                let left_true = self.satisfies_expression(left, Expectative::True);
                let right_true = self.satisfies_expression(right, Expectative::True);

                left_true && right_true
            }

            Expression::Xor (left, right)=> {
                let left_true = self.satisfies_expression(left, Expectative::True);
                let left_false = self.satisfies_expression(left, Expectative::False);

                let right_true = self.satisfies_expression(right, Expectative::True);
                let right_false = self.satisfies_expression(right, Expectative::False);

                (left_true && right_false) || (left_false && right_true)
            }
        }
    }

    pub fn satisfies (&self, expectative: Expectative) -> bool {
        self.satisfies_expression(self.expression, expectative)
    }
}

impl<'a> Expression {
    pub fn general_satisfability (&'a self) -> GeneralSatisfability<'a> {
        GeneralSatisfability::new(self)
    }
}

#[cfg(test)]
mod test {
    // use crate::expression;

    use super::*;

    #[test]
    fn var_expression_should_satisfies_true () {
        let expression = Expression::Var("a".to_string());
        
        assert!(expression.general_satisfability().satisfies(Expectative::True));
    }

    #[test]
    fn var_expression_should_satisfies_false () {
        let expression = Expression::Var("a".to_string());
        
    assert!(expression.general_satisfability().satisfies(Expectative::False));
    }

    #[test]
    fn not_expression_should_satisfies_true () {
        let expression = Expression::Not(
            Box::new(
                Expression::Var("a".to_string())
            )
        );
        
        assert!(expression.general_satisfability().satisfies(Expectative::True));
    }

    #[test]
    fn not_expression_should_satisfies_false () {
        let expression = Expression::Not(
            Box::new(
                Expression::Var("a".to_string())
            )
        );
        
        assert!(expression.general_satisfability().satisfies(Expectative::False));
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
        
        assert!(expression.general_satisfability().satisfies(Expectative::True));
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
        
        assert!(expression.general_satisfability().satisfies(Expectative::False));
    }
}