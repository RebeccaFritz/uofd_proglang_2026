// Rebecca
enum Primitive {
    Add,
    Multiply,
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
        Primitive::Number(_) => 0
    } 
}

fn main() {

    let result: bool = true;
    println!("{result}")

}
