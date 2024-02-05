pub fn is_happy(mut n: i32) -> bool{
    loop {
        let mut s = 0;
        while n > 0 {
            s += (n % 10).pow(2);
            n = n / 10;
        }

        match s {
            1 | 4 => break s == 1,
            _ => n = s,
        }
    }
}

fn main() {
    let n = 11479839;
    let number_emotion = if is_happy(n) { "happy" } else { "not happy" };
    println!("The number is {}!", number_emotion);
}



















//                 First Try
// fn get_next(mut n: i32) -> i32 {
//     let mut total_sum = 0;
//     while n > 0 {
//         let digit = n % 10;
//         n = n / 10;
//         total_sum += digit * digit;
//         println!("n = {}, digit = {}, total_sum = {}", n, digit, total_sum);
//     }
//     total_sum
// }

// fn main() {
//     let mut n = 710284712;
//     while n != 1 && n != 4 {
//         n = get_next(n);
//         println!("n = {}", n);
//     }
//     let number_emotion = if n == 1 { "happy" } else { "not happy" };
//     println!("The number is {}!", number_emotion);
// }