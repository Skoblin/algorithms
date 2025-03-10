fn main() {
    let number = 7;
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    match binary_search(number, numbers) {
        Some(index) => println!("Индекс найденного элемента: {}", index),
        None => println!("В массиве нет заданного элемента"),
    }

}

fn binary_search(number: i32, mut numbers: Vec<i32>) -> Option<usize> {
    let mut right: usize = numbers.len();
    let mut left: usize = 0;
    numbers.sort();
    while left < right {
        let mid: usize = left + (right - left) / 2;
        if number == numbers[mid] {
            return Some(mid); 
        } else if numbers[mid] < number {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    None

}