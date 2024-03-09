use std::{thread, time};

fn print_row(row: &Vec<i32>) -> () {
    let wait_time = time::Duration::from_secs(2);

    row.iter().for_each(|int| {
        println!("{}", int);
        thread::sleep(wait_time);
    });
}

fn main() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    matrix.iter().for_each(print_row);
}
