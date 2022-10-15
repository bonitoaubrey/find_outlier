fn main() {
    let list = [25723, -11015, 7, 8];
    let number = find_outlier(&list);
    println!("The value of number is: {number}");
}

fn find_outlier(values: &[i32]) -> i32 {
    let mut odd = values[0] % 2 == 0;

    if (odd != (values[1] % 2 == 0)) && (odd != (values[2] % 2 == 0)){
        odd = !odd;
    }

    println!("{odd}");

    for i in values {
        match (i % 2 != 0) == odd {
            true => return *i,
            false => continue,
        }
    }

    8
}
