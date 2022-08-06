fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

fn main() {
    let num_list = vec![1, 2, 34, 5, 6];
    let result = largest(&num_list);

    println!("The largest number is {}", result);
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}
