fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {

    let numbers = [2, 3, 5, 7, 12, 15, 20, 30, 39, 44];

    for &num in numbers.iter() {
        // divisible by both 3 and 5 
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{}: Fizz", num);
        } else if num % 5 == 0 {
            println!("{}: Buzz", num);
        } else {
            // Otherwise, print whether the number is even or odd.
            if is_even(num) {
                println!("{}: Even", num);
            } else {
                println!("{}: Odd", num);
            }
        }
    }

    //sum all the numbers in the array
    let mut index = 0;
    let mut sum = 0;
    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }
    println!("Sum of all numbers: {}", sum);

    //find the largest number
    let mut largest = numbers[0];
    let mut i = 1;
    loop {
        if i >= numbers.len() {
            break;
        }
        if numbers[i] > largest {
            largest = numbers[i];
        }
        i += 1;
    }
    println!("Largest number: {}", largest);
}

