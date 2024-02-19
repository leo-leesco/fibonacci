fn fibo(values: &mut Vec<i64>) -> () {
    match values.len() {
        0 => values.push(0),
        1 => values.push(1),
        _ => values.push(values[values.len() - 1] + values[values.len() - 2]),
    }
}

fn main() {
    let mut fibo_vector: Vec<i64> = vec![];
    for _ in 0..10 {
        fibo(&mut fibo_vector);
    }
    println!("{fibo_vector:?},{}", fibo_vector.len());
}
