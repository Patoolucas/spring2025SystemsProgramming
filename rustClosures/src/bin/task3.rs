fn process_vector_map<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    vec.into_iter().map(f).collect()
}

fn process_vector_for_loop<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    let mut result = Vec::new();
    for x in vec {
        result.push(f(x));
    }
    result
}

fn main() {
    let numbers = vec![1, 2, 3];

    //map
    let doubled = process_vector_map(numbers.clone(), |x| x * 2);
    let replaced = process_vector_map(numbers.clone(), |x| if x > 2 { 0 } else { x });

    println!("Doubled (map): {:?}", doubled);
    println!("Replaced (map): {:?}", replaced);

    //for loop
    let doubled_loop = process_vector_for_loop(numbers.clone(), |x| x * 2);
    let replaced_loop = process_vector_for_loop(numbers, |x| if x > 2 { 0 } else { x });

    println!("Doubled (for loop): {:?}", doubled_loop);
    println!("Replaced (for loop): {:?}", replaced_loop);
}
