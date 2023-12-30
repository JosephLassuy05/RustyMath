pub mod tests;

// rounds the number to the nearest decimal place 0 being the integer
pub fn round(number: f64, decimal_place: u16) -> f64 {
    (number * 10_f64.powf(decimal_place.into())).round() / 10_f64.powf(decimal_place.into())
}

// finds the average of all the numbers
pub fn mean(numbers: Vec<f64>) -> f64 {
    let mut total: f64 = 0.0;
    for number in &numbers {
        total += number;
    }
    total / numbers.len() as f64
}

// The middle number in a data set ordered
pub fn median(mut numbers: Vec<f64>) -> f64 {
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let length = numbers.len();
    if length % 2 == 0 {
        return (numbers[length / 2] + numbers[length / 2 - 1]) / 2.0;
    } else {
        let test = length / 2;
        println!("{}, {}, {}", numbers.len(), test, numbers[length / 2]);
        return numbers[length / 2];
    }
}

// Finds the number that is most common
pub fn mode() {
    todo!()
}

// Finds the difference between the highest number in the dataset and the lowest
pub fn range(mut numbers: Vec<f64>) -> f64 {
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let lowest = numbers[0];
    let highest = numbers[numbers.len() - 1];
    println!("lowest: {} highest: {}", lowest, highest);
    return highest - lowest;
}
