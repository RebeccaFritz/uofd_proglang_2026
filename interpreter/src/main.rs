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

fn evaluate_addition(expression: &Expression) -> Expression {
    if let Expression::Addition(expressions) = expression {
        match expressions[0] {
            Expression::Integer(_) => evaluate_add_integers(expressions),
            _ => panic!("I only know integers at the moment...")
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
}

