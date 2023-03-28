pub fn run() {
    let mut numbers = vec![2, 3, 4, 6, 12, 34, 4, 1, 2, 43, 23];

    println!("{:?}", numbers);

    numbers.sort_by(|a, b| b.cmp(a));

    println!("{:?}", numbers);
    println!("Length: {:?}", numbers.len());

    if numbers.len() == 0 {
        println!("The numbers array is empty");
    } else {
        let median_index: usize = ((numbers.len() as f32) / 2.0).floor() as usize;
        println!("Median Index: {median_index}");
        println!("The median element in the array is: {:?}", numbers[median_index]);
    }
}