// Rebecca Fritz
#[derive(Clone)]
#[derive(Debug)]
pub enum Expression {
    Add(Vec<Expression>),
    Subtract(Vec<Expression>),
    Multiply(Vec<Expression>),
    Divide(Vec<Expression>),
    Variable(String),
    Number(i32)
}

pub struct Environment {
    key: String,
    value: Expression
}

impl Environment {
    fn value_for_key(self: &Environment, key: &String) -> &Expression {
        if &self.key == key {
            &self.value
        } else {
            panic!("key not found in environment");
        }
    }
    fn new() -> Environment {
        Environment {
            key: String::from(""),
            value: Expression::Number(0),
        }
    }
}

pub fn evaluate_addition(add: &Expression, environment: &Environment) -> i32 {
    if let Expression::Add(expressions) = add {
        let iter = expressions.iter();
        iter.fold(0, |total, next| total + evaluate(next, environment))
    } else {
        panic!("Addition not provided");
    }
}

pub fn evaluate_multiplication(mult: &Expression, environment: &Environment) -> i32 {
    if let Expression::Multiply(expressions) = mult {
        let iter = expressions.iter();
        iter.fold(1,|total, next| total * evaluate(next, environment))
    } else {
        panic!("Multiply not provided");
    }
}

pub fn evaluate_division(div: &Expression, environment: &Environment) -> i32 {
    if let Expression::Divide(expressions) = div {
        if expressions.len() != 2 {
            panic!("There must only be two operands");
        }
        let mut iter = expressions.iter();
        let first = iter.next().unwrap();
        iter.fold(evaluate(first, environment), |total, next| total / evaluate(next, environment))
    } else {
        panic!("Divide not provided")
    }
}

pub fn evaluate_subtraction(sub: &Expression, environment: &Environment) -> i32 {
    if let Expression::Subtract(expressions) = sub {
            let mut iter = expressions.iter();
            let first = iter.next().unwrap();
            iter.fold(evaluate(first, environment), |total, next| total - evaluate(next, environment))
    } else {
        panic!("Not subtraction");
    }
}

fn evaluate(expression: &Expression, environment: &Environment) -> i32 {
    match expression {
        Expression::Add(_) => evaluate_addition(expression, environment),
        Expression::Subtract(_) => evaluate_subtraction(expression, environment),
        Expression::Multiply(_) => evaluate_multiplication(expression, environment),
        Expression::Divide(_) => evaluate_division(expression, environment),
        Expression::Variable(key) => {
            let expr = environment.value_for_key(key);
            evaluate(expr, environment)
        }
        Expression::Number(val) => *val,
    }
}

pub fn print_addition(add: &Expression, environment: &Environment) {
    if let Expression::Add(expressions) = add {
        let iter = expressions.iter();
        print!("{}", &String::from(" (+ "));
        iter.for_each(|item| {
            print_expression(item, environment);
        });
        print!("{}", &String::from(")"));
    } else {
        panic!("Not addition");
    }
}

pub fn print_multiplication(mult: &Expression, environment: &Environment) {
    if let Expression::Multiply(expressions) = mult {
        let iter = expressions.iter();
        print!("{}", &String::from(" (* "));
        iter.for_each(|item| {
            print_expression(item, environment);
        });
        print!("{}", &String::from(")"));
    } else {
        panic!("Not multiplication");
    }
}

pub fn print_division(div: &Expression, environment: &Environment) {
    if let Expression::Divide(expressions) = div {
        let iter = expressions.iter();
        print!("{}", &String::from(" (/ "));
        iter.for_each(|item| {
            print_expression(item, environment);
        });
        print!("{}", &String::from(")"));
    } else {
        panic!("Not division");
    }
}

pub fn print_subtraction(sub: &Expression, environment: &Environment) {
    if let Expression::Subtract(expressions) = sub {
        let iter = expressions.iter();
        print!("{}", &String::from(" (- "));
        iter.for_each(|item| {
            print_expression(item, environment);
        });
        print!("{}", &String::from(")"));
    } else {
        panic!("Not subtraction");
    }
}

fn print_expression(expression: &Expression, environment: &Environment) {
    match expression {
        Expression::Add(_) => print_addition(expression, environment),
        Expression::Subtract(_) => print_subtraction(expression, environment),
        Expression::Multiply(_) => print_multiplication(expression, environment),
        Expression::Divide(_) => print_division(expression, environment),
        Expression::Variable(key) => {
            let expr = environment.value_for_key(key);
            print_expression(expr, environment)
        },
        Expression::Number(val) => {
            print!(" {}", *val);
        }
    }
}

fn main() {
    // let mut expressions = Vec::new();
    // expressions.push(Expression::Number(3));
    // expressions.push(Expression::Number(4));
    // expressions.push(Expression::Number(5));
    // let add = Expression::Add(expressions);
    // let multiply = Expression::Multiply(vec![add, Expression::Number(2)]);
    // let result = evaluate(&multiply, &Environment::new());
    // println!("The result is {result}");

    let expression = Expression::Multiply(vec![
            Expression::Number(8),
            Expression::Divide(vec![
                Expression::Number(5),
                Expression::Add(vec![
                    Expression::Number(3),
                    Expression::Subtract(vec![
                        Expression::Number(2),
                        Expression::Number(1)
                    ])
                ])
            ])
        ]
    );
    print_expression(&expression, &Environment::new()); // should get "(* 8 (/ 5 (+ 3 ( - 2 1))))"


    println!(" ");
    let mut new_env = crate::Environment {
            key: String::from("volcanalis"),
            value: crate::Expression::Number(5)
        };


    let expression = Expression::Multiply(vec![
            Expression::Number(3),
            Expression::Divide(vec![
                crate::Expression::Variable(String::from("volcanalis")),
                Expression::Add(vec![
                    Expression::Number(8),
                    Expression::Subtract(vec![
                        crate::Expression::Variable(String::from("volcanalis")),
                        Expression::Number(3)
                    ])
                ])
            ])
        ]
    );
    print_expression(&expression, &new_env); // should get "(* 3 (/ 5 (+ 8 ( - 5 3))))"

}

#[cfg(test)]
mod tests {
use crate::{evaluate_addition, evaluate_subtraction, evaluate_multiplication, evaluate_division, Expression};
    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }

    #[test]
    fn test_basic_addition() {
        // arrange
        let values = vec![Expression::Number(2), Expression::Number(2)];
        // act
        let sum = evaluate_addition(&Expression::Add(values), &crate::Environment::new());
        // assert
        assert_eq!(sum, 4);
    }

    #[test]
    fn test_basic_subtraction() {
        // arrange
        let values = vec![Expression::Number(2), Expression::Number(2)];
        // act
        let difference = evaluate_subtraction(&Expression::Subtract(values), &crate::Environment::new());
        // assert
        assert_eq!(difference, 0);
    }
    #[test]
    fn test_basic_addition_not5() {
        // arrange
        let values = vec![Expression::Number(2), Expression::Number(2)];
        // act
        let sum = evaluate_addition(&Expression::Add(values), &crate::Environment::new());
        // assert
        assert_ne!(sum, 5);
    }

    #[test]
    fn test_multiplication_with_addition() {
        // arrange
        let addition_values = vec![
            Expression::Number(2), 
            Expression::Number(2)];
        let mut multiplication_values = vec![
            Expression::Number(3), 
            Expression::Number(4), 
            Expression::Number(5),
        ];
        // act
        let sum = evaluate_addition(&Expression::Add(addition_values), &crate::Environment::new());
        multiplication_values.push(Expression::Number(sum));
        let product = evaluate_multiplication(&Expression::Multiply(multiplication_values), &crate::Environment::new());
        // assert
        assert_eq!(product, 240);
    }

    #[test]
    fn test_basic_division() {
        // arrange
        let values = vec![Expression::Number(216), Expression::Number(6)];
        // act
        let quotient = evaluate_division(&Expression::Divide(values), &crate::Environment::new());
        // assert
        assert_eq!(quotient, 36);
    }

    #[test]
    fn test_new_environment() {
        // arrange
        // act
        let mut new_env = crate::Environment::new();
        // assert
        let expr = new_env.value;
        if let crate::Expression::Number(value) = expr {
            assert_eq!(value, 0);
        } else {
            assert_eq!(1,0);
        }
    }

    #[test]
    fn test_value_for_key() {
        // arrange
        let mut new_env = crate::Environment::new();
        new_env.key = String::from("foo");
        new_env.value = crate::Expression::Number(2);
        // act
        let expr = new_env.value_for_key(&String::from("foo"));
        // assert
        if let crate::Expression::Number(value) = expr {
            assert_eq!(*value, 2);
        } else {
            assert_ne!(1,1);
        }
    }

    #[test]
    fn test_addition_with_variable() {
        // arrange
        let mut new_env = crate::Environment {
            key: String::from("x"),
            value: crate::Expression::Number(6)
        };
        let vec = vec![crate::Expression::Number(7), crate::Expression::Variable(String::from("x"))];
        let add = crate::Expression::Add(vec);
        // act
        let value = crate::evaluate(&add, &new_env);
        // assert
        assert_eq!(value, 13);
    }

    #[test]
    fn test_division_with_variable() {
        // arrange
        let mut new_env = crate::Environment {
            key: String::from("easter"),
            value: crate::Expression::Number(90)
        };
        let vec = vec![crate::Expression::Number(3150), crate::Expression::Variable(String::from("easter"))];
        let divide = crate::Expression::Divide(vec);
        // act
        let value = crate::evaluate(&divide, &new_env);
        // assert
        assert_eq!(value, 35);
    }

    #[test]
    #[should_panic]
    fn test_division_with_3_operands() {
        // arrange
        let mut new_env = crate::Environment {
            key: String::from("sunday"),
            value: crate::Expression::Number(90)
        };
        let vec = vec![
            crate::Expression::Number(3150), 
            crate::Expression::Variable(String::from("sunday")), 
            crate::Expression::Number(5)];
        let divide = crate::Expression::Divide(vec);
        // act
        let value = crate::evaluate(&divide, &new_env);
        // assert
        // assert_eq!(value, 7); // this actually does work if you remove the panic condition from the evaluate_division method
    }

}