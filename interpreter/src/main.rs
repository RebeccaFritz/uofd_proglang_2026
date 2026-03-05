// Rebecca
enum Primitive {
    Add,
    Multiply,
    Subtract,
    Number(i32)
}

fn eval_prim(primitive: &Primitive) -> i32 {
    match primitive {
        Primitive::Number(val) => *val,
        _ => 0
    }
}

fn evaluate(primitives: Vec<Primitive>) -> i32 {
    match primitives[0] {
        Primitive::Add => { eval_prim(&primitives[1]) + 
            eval_prim(&primitives[2]) },
        Primitive::Multiply => { eval_prim(&primitives[1]) *
            eval_prim(&primitives[2]) },
        Primitive::Subtract => { eval_prim(&primitives[1]) - 
            eval_prim(&primitives[2]) },
        Primitive::Number(_) => 0
    } 
}

fn main() {

    // this is using Polish Notation (Prefix notiation) so (- 3 4) is equivalent to (3 - 4)
    let mut primitives = Vec::<Primitive>::new();
    primitives.push(Primitive::Subtract); 
    primitives.push(Primitive::Number(3));
    primitives.push(Primitive::Number(4));
    let result = evaluate(primitives);
    println!("The result is {result}");

}



