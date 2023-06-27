/* 
Goal: Generate the nth Fibonacci number:
https://doc.rust-lang.org/book/ch03-05-control-flow.html#:~:text=nicer%2C%20isn%E2%80%99t%20it%3F-,Summary,-You%20made%20it
*/

// xn = xn−1 + xn−2, or the Combinational Method
fn bizonacci(nth: i32) -> Vec<i32> {
    let mut seed = vec![0,1];
    for _number in 0..nth - 1 {
        let combined = seed[seed.len() - 1] + seed[seed.len() - 2];
        seed.push(combined);
    }

    println!("Fibonacci n{:?} via combinational method is {:?}", nth, seed.last());
    return seed;
}

// Golden Ratio Method
fn g_ratio(nth: i32) -> i64{
    let ratio: f64 = 1.618034;
    let result =
    (ratio.powf(nth.into()) - (1.0 - ratio).powf(nth.into())) / 
                    2.2360679775; // sqrt(5)

    return result as i64;
}

fn main() {
    bizonacci(9);
    println!("Calculation using Golden Ratio: {:?}", g_ratio(9))
}
