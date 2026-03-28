fn main() {
    let v = vec![0,1,2,3,4,5,6,7,8,9];

    let total: i32 = v.into_iter()
        .filter(|x| x%2 == 1)
        .map(|x| x*2)
        .fold(0, |total, x| total + x);

    println!("{total}");

}
