// Rebecca Fritz
#[derive(Debug)]
#[derive(PartialEq)]

enum Expression {
    Integer(i32),
    FixedPoint(i32, i32),
    Addition(Vec<Expression>)
}

fn evaluate_add_integers(expressions: &Vec<Expression>) -> Expression {
    let mut total = 0;
    for each in expressions {
        if let Expression::Integer(value) = each {
            total = total + value;
        } else {
            panic!("non-integer provided in vector");
        }
    }
    Expression::Integer(total)
} 

fn evaluate_add_fixed_point(expressions: &Vec<Expression>) -> Expression {
    let mut total_whole = 0;
    let mut total_frac = 0;
    for each in expressions {
        if let Expression::FixedPoint(whole, frac) = each {
            total_whole = total_whole + whole;
            total_frac = total_frac + frac;
            if total_frac > 99 {
                total_whole = total_whole + 1;
                total_frac = total_frac - 100;
            }
        } else {
            panic!("non-fixed-point value provided in vector");
        }
    }
    
    Expression::FixedPoint(total_whole, total_frac)
}

fn evaluate_addition(expression: &Expression) -> Expression {
    if let Expression::Addition(expressions) = expression {
        match expressions[0] {
            Expression::Integer(_) => evaluate_add_integers(expressions),
            Expression::FixedPoint(_, _) => evaluate_add_fixed_point(expressions),
            _ => panic!("Addition is only provided for integers and fixed-points")
        }
    } else {
        panic!("not addition");
    }
}

fn evaluate_integer(expression: &Expression) -> f64 {
    if let Expression::Integer(value) = expression {
        *value as f64
    } else {
        panic!("expected integer, got some other type");
    }
}

fn evaluate_fixed_point(expression: &Expression) -> f64 {
    if let Expression::FixedPoint(whole,frac) = expression {
        return (*whole as f64) + ((*frac as f64) / 100.0);
    } else {
        panic!("oh no, it's not fixed point");
    }
}

fn evaluate(expression: &Expression) -> f64 {
    match expression {
        Expression::Addition(_) => evaluate(&evaluate_addition(expression)),
        Expression::Integer(_) => evaluate_integer(expression),
        Expression::FixedPoint(_,_) => evaluate_fixed_point(expression)
    }
}

fn main() {
    todo!("write a main function");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_anything_works() {
        assert!(true, "it worked");
    }

    #[test]
    fn test_simple_addition() {
        // arrange
        let expr = crate::Expression::Addition(vec![
            crate::Expression::Integer(2),
            crate::Expression::Integer(2)
        ]);
        // act
        let val = crate::evaluate(&expr);
        // assert
        assert_eq!(val, 4.0, "expressions not equal");
    }

    #[test]
    fn test_evaluate_fixed_point() {
        // arrange
        let expr = crate::Expression::FixedPoint(2,5);
        // act
        let val = crate::evaluate_fixed_point(&expr);
        // assert
        assert_eq!(val, 2.05, "expressions not equal");
    }

    #[test]
    fn test_fixed_point_addition() {
        // arrange
        let expr = crate::Expression::Addition(vec![
            crate::Expression::FixedPoint(2,3),
            crate::Expression::FixedPoint(4,25)
        ]);
        let expected = crate::Expression::FixedPoint(6, 28);
        // act
        let val = crate::evaluate_addition(&expr);
        // assert
        assert_eq!(val, expected, "expressions not equal");
    }

    #[test]
    fn test_fixed_point_addition_direct() {
        // arrange
        let expr = vec![
            crate::Expression::FixedPoint(4,11),
            crate::Expression::FixedPoint(5,13)
        ];
        let expected = crate::Expression::FixedPoint(9, 24);
        // act
        let val = crate::evaluate_add_fixed_point(&expr);
        // assert
        assert_eq!(val, expected, "expressions not equal");
    }

    #[test]
    fn test_fixed_point_addition_big_decimals() {
        // arrange
        let expr = vec![
            crate::Expression::FixedPoint(4,92),
            crate::Expression::FixedPoint(5,56)
        ];
        let expected = crate::Expression::FixedPoint(10, 48);
        // act
        let val = crate::evaluate_add_fixed_point(&expr);
        // assert
        assert_eq!(val, expected, "expressions not equal");
    }
}

